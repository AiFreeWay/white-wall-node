pub struct Error {
    id: u8,
    description: String
}

pub enum ErrorTypes {
    GenerateKeypair = 1
}

impl Error {
    
    pub fn keypair_error() -> Error {
        return Error {
            id: ErrorTypes::GenerateKeypair as u8,
            description: String::from("Error keypair generation")
        }
    }
}