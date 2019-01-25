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
    
    pub fn is_have_account(&self) -> Result<bool, Exception> {
        return self.get_storage().get_private_key()
            .map(|result| result.is_some())
    }
    
    fn get_storage(&self) -> &T {
        return &self.storage
    }
}

