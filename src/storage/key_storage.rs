use configuration;
use ed25519_dalek::Keypair;
use exception_handler::Error;
use std::fs::File;


struct Key {
    key: String,
}

impl Key {
    
    pub fn new(path: String, keypair: Keypair) {
        save_key_file(path, keypair)
    }
    
    pub fn from_path(path: String) -> Result<Key, Error> {
        let key = Key {
            key: read_key_file(path),
        };
        
        return Ok(key);
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

impl KeyStorage {
    
    pub fn create(keyseed: String) -> Result<KeyStorage, Error> {
        let key_path = configuration::get_config().get_key_path();
        
        return match Key::new(key_path, keyseed) {
            Ok(key) => KeyStorage { key: key },
            Err(err) => err,
        }
    }
    
    pub fn open() -> Result<KeyStorage, Error> {
        let key_path = configuration::get_config().get_key_path();
        
        return match Key::from_path(key_path) {
            Ok(key) => KeyStorage { key: key },
            Err(err) => err,
        }
    }
    
    pub fn get_keypair(&self) -> Result<Keypair, Error> {
        let generate_result = Keypair::from_bytes(self.key.get_key_bytes());
        return match generate_result {
            Ok(keypair) => Ok(keypair),
            Err(err) => Err(Error::generate_keypair_error(err.to_string()))
        }
    }
    
    fn read_key_file(path: str) -> Result<String, Error> {
        match File::open(path) {
            Ok(file) => {},
            Err(error)
        }
    }
    
    fn save_key_file(path: String, keyseed: String) -> Result<String, Error> {
        return std::fs::remove_file(path)
            .map_err(|err| -> Error::save_keyseed_error(err.to_string()))
            .and(|_| -> File::create(keyseed))
            .and(|file| -> file::write_fmt(!format(path)))
            .and(|_| -> path)
    }
}