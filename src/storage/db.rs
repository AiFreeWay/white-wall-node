extern crate rocksdb;

use self::rocksdb::{DB, DBVector, Error};


pub type Key = String;
pub type Value = String;

pub struct Database {
    database: DB
}

impl Database {
    pub fn new() -> Database {
        let db = match DB::open_default("path/for/rocksdb/storage") {
            Ok(db) => db,
            Err(e) => panic!(e),
            
        };
        
        return Database {
            database: db
        }     
    }
    
    pub fn write(&self, key: Key, value: Value) {
        self.get_db().put(key.as_bytes(), value.as_bytes());
    }
    
    pub fn read(&self, key: Key) -> Option<Value> {
        return self.get_db()
            .get(key.as_bytes())
            .ok()
            .map(|value| value);
    }
    
    fn get_db(&self) -> &DB {
        &self.database  
    }
}