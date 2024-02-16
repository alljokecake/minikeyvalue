use std::path::PathBuf;

use anyhow::{Error, Result};
use leveldb::database::Database;
use leveldb::options::Options;

pub fn from_path(path: PathBuf) -> Result<Database<i32>, Error> {
    let mut options = Options::new();
    options.create_if_missing = true;

    Ok(Database::open(&path, options)?)
}

pub enum Deleted {
    No = 0,
    Soft = 1,
    Hard = 2,
}

pub struct Record {
    rvolumes: Vec<String>,
    deleted: Deleted,
    hash: String,
}

impl Default for Record {
    fn default() -> Self {
        Self::new()
    }
}

impl Record {
    pub fn new() -> Self {
        Record {
            rvolumes: Vec::new(),
            deleted: Deleted::Hard,
            hash: String::new(),
        }
    }

    pub fn record_from(&self) -> &[u8] {
        todo!()
    }

    pub fn to_record(_data: &[u8]) -> Self {
        todo!()
    }
}
