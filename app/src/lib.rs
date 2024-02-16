#![allow(dead_code)]

mod database;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::{anyhow, Error, Result};
use database::{from_path, Record};
use leveldb::database::Database;
use leveldb::kv::KV;
use leveldb::options::{ReadOptions, WriteOptions};

use commands::Options;

pub struct App {
    db: Arc<Database<i32>>,
    mlock: Mutex<()>,
    lock: HashMap<String, ()>,
    uploadids: HashMap<String, bool>,
    volumes: Vec<String>,
    fallback: String,
    replicas: u64,
    subvolumes: u64,
    protect: bool,
    md5sum: bool,
    voltimeout: Duration,
}

impl App {
    pub fn unlock_key(&mut self, key: &str) {
        let _guard = self.mlock.lock().unwrap();
        self.lock.remove(key);
    }

    pub fn lock_key(&mut self, key: &str) -> bool {
        let _guard = self.mlock.lock().unwrap();
        if self.lock.contains_key(key) {
            return false;
        }
        self.lock.insert(key.to_string(), ());
        true
    }

    // TODO: very very very bad practice
    pub fn get_record(&self, key: &str) -> Record {
        let options = ReadOptions::new();
        let key: i32 = key.trim().parse().expect("key != i32");

        match self.db.get(options, key) {
            Ok(Some(v)) => Record::to_record(&v),
            _ => Record::new(),
        }
    }

    // TODO: very very very bad practice
    pub fn put_record(&self, key: &str, rec: Record) -> bool {
        let options = WriteOptions::new();
        let key: i32 = key.trim().parse().expect("key != i32");

        self.db.put(options, key, Record::record_from(&rec)).is_ok()
    }

    pub fn new() -> Result<Self, Error> {
        let pvolumes = Options::get_string(Options::turner().pvolumes)?;
        let volumes: Vec<String> =
            pvolumes.split(',').map(String::from).collect();

        let replicas = Options::turner().replicas;

        if volumes.len() < replicas.try_into().unwrap() {
            return Err(anyhow!("Need at least as many volumes as replicas"));
        }

        let db = from_path(Options::turner().pdb)?.into();
        let fallback = Options::get_string(Options::turner().fallback)?;
        let md5sum = Options::turner().md5sum;
        let subvolumes = Options::turner().subvolumes;
        let protect = Options::turner().protect;
        let voltimeout = Duration::from_secs(Options::turner().voltimeout);

        Ok(App {
            mlock: Mutex::new(()),
            lock: HashMap::new(),
            uploadids: HashMap::new(),

            db,
            volumes,
            fallback,
            replicas,
            subvolumes,
            protect,
            md5sum,
            voltimeout,
        })
    }
}
