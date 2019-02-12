use ed25519_dalek::Keypair;
use exception_handler::Error;


struct KeyStorage {
    dir: String
}

trait Store {
    fn getPrivateKey(password: str);
    fn getPublicKey(password: str);
    
    fn createKeyPair(password: str);
}

impl KeyStorage {
    
    pub fn new() -> KeyStorage {
        return KeyStorage {
            dir: String::from("~/WallData/storage")
        }
    }
    
    pub fn create_key_pair(password: &str) -> Option<Error> {
        let generate_result = Keypair::from_bytes(&[1, 2, 3]);
        match generate_result {
            Ok(keypair) => {
                println!("{:?}", keypair.public);
            },
            Err(e) => return Some(Error::keypair_error())
        }
        //generate keyPair from password
        None
    }
    
    fn is_key_pair_valid(paswword: &str) -> bool {
        true
    }
}