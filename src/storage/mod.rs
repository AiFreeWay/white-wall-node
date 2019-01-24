pub mod keys;

use self::keys::PrivateKey;
use business_logic::exceptions::Exception;


pub fn get_private_key() -> Result<Option<String>, Exception> {
    return Ok(Some(
        String::from(PrivateKey::new(Vec::new()).get_key())
    ))
}