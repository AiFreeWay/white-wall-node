pub struct PrivateKey {
    key: String,
    creation_date: u32,
    date_of_change: u32,
}

impl PrivateKey {
    
    pub fn new(data: Vec::<u8>) -> PrivateKey {
        return PrivateKey {
            key: String::from(""),
            creation_date: 0,
            date_of_change: 0
        }
    }
    
    pub fn get_key(&self) -> &str {
        return &self.key
    }
    
    pub fn is_was_changed(&self) -> bool {
        return true
    }
}