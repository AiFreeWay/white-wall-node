///Exceptions
pub mod exceptions;

use storage;
use self::exceptions::Exception;


pub fn is_have_account() -> Result<bool, Exception> {
    return storage::get_private_key()
        .map(|result| result.is_some())
}