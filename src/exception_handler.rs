pub struct Error {
    id: u8,
    description: String
}

pub enum ErrorTypes {
    GenerateKeypair    = 1,
    SaveKeySeed = 2,
}

impl Error {
    
    pub fn generate_keypair_error(description: String) -> Error {
        let description = format!("Error keypair generation: {}", description);
        return Error {
            id: ErrorTypes::GenerateKeypair as u8,
            description: description
        }
    }
    
    pub fn save_keyseed_error(description: String) -> Error {
        let description = format!("Error keyseed saving: {}", description);
        return Error {
            id: ErrorTypes::SaveKeySeed as u8,
            description: description
        }
    }
}