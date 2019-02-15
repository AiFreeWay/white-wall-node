pub struct Key {
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
    
    pub fn is_was_changed(&self) -> bool {
        return true
    }
}