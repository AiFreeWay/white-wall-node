use configuration;
use ed25519_dalek::Keypair;
use storage::key::Key;
use exception_handler::Error;


struct KeyStorage {
    key: Key
}

impl KeyStorage {
    
    pub fn new() -> KeyStorage {
        let key_path = configuration::get_config().get_key_path();
        
        return KeyStorage {
            key: Key::from_path(key_path)
        }
    }
    
    pub fn get_keypir(&self) -> Result<Keypair, Error> {
        let generate_result = Keypair::from_bytes(self.key.get_key_bytes());
        return match generate_result {
            Ok(keypair) => Ok(keypair),
            Err(e) => Err(Error::keypair_error())
        }
    }
}