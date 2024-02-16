use std::path::PathBuf;

use anyhow::{Error, Result};
use leveldb::database::Database;
use leveldb::options::Options;

pub fn from_path(path: PathBuf) -> Result<Database<i32>, Error> {
    let mut options = Options::new();
    options.create_if_missing = true;

    Ok(Database::open(&path, options)?)
}

#[allow(unused)]
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
        let mut cc = String::new();

        match self.deleted {
            Deleted::No => {},
            Deleted::Soft => {
                cc.push_str("DELETED");
            },
            Deleted::Hard => {
                panic!("Can't put HARD delete in the database");
            },
        }
        if self.hash.len() == 32 {
            cc.push_str(&format!("HASH{}", self.hash));
        }

        let rvolumes_str = self.rvolumes.join(",");
        cc.push_str(&rvolumes_str);

        todo!()
    }

    pub fn to_record(_data: &[u8]) -> Self {
        todo!()
    }
}
