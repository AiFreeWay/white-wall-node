/// PrivateKey model
pub mod key;
/// Event model
pub mod event;
/// Event selector
pub mod event_selector;

use self::key::PrivateKey;
use self::event::Event;
use self::event_selector::EventSelector;
use business_logic::exceptions::Exception;
use rocksdb::DB;


pub trait Storage {
    fn write(&self, key: String, value: String);
    fn read(&self, key: String) -> Result<String, Exception>;
}

pub struct StorageImpl {
    database: DB
}

impl StorageImpl {
    pub fn new() -> StorageImpl {
         let db = DB::open_default("path/for/rocksdb/storage").unwrap();
         return StorageImpl {
             database: db
         }
    }
    
    fn get_db(&self) -> &DB {
        return &self.database
    }
}

impl Storage for StorageImpl {
    fn write(&self, key: String, value: String) {
        self.get_db().put(key, value)
    }
    
    fn read(&self, key: String) -> Result<String, Exception> {
        return Ok(String::new())
    }
}
