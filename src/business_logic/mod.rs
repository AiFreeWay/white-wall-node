///Exceptions
pub mod exceptions;
///Unit tests
#[cfg(test)]
mod tests;

use self::exceptions::Exception;
use storage::Storage;


pub struct Executor<T> {
    storage: T
}

impl<T> Executor<T> where T: Storage {
    
    pub fn from(storage: T) -> Executor<T> {
        return Executor {
            storage: storage
        }
    }
}

