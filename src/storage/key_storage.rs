use configuration;
use ed25519_dalek::Keypair;
use exception_handler::Error;


struct Key {
    key: String,
    creation_date: u32,
    date_of_change: u32,
}

impl Key {
    
    pub fn from_path(path: String) -> Key {
        return Key {
            key: String::from(""),
            creation_date: 0,
            date_of_change: 0
        }
    }
    
    pub fn get_key(&self) -> &str {
        return &self.key
    }
    
    pub fn get_key_bytes(&self) -> &[u8] {
        return &self.key.as_bytes()
    }
}

struct KeyStorage {
    key: Key
}

impl  KeyStorage {
    
    pub fn new() -> KeyStorage {
        let key_path = configuration::get_config().get_key_path();
        
        return KeyStorage {
            key: Key::from_path(key_path)
        }
    }
    
    pub fn get_keypair(&self) -> Result<Keypair, Error> {
        let generate_result = Keypair::from_bytes(self.key.get_key_bytes());
        return match generate_result {
            Ok(keypair) => Ok(keypair),
            Err(e) => Err(Error::keypair_error())
        }
    }
    
    pub fn is_was_changed(&self) -> bool {
        return true
    }
}