pub mod keys;

use self::keys::PrivateKey;
use business_logic::exceptions::Exception;


pub trait Storage {
    fn get_private_key(&self) -> Result<Option<String>, Exception>;
}

pub struct StorageImpl {}

impl Storage for StorageImpl {
    
    fn get_private_key(&self) -> Result<Option<String>, Exception> {
        return Ok(Some(
            String::from(PrivateKey::new(Vec::new()).get_key())
        ))
    }
}
