pub trait Config {
    fn get_database_path(&self) -> String;
    fn get_key_path(&self) -> String;
}

struct ConfigInternal {}

impl Config for ConfigInternal {
    
    fn get_database_path(&self) -> String {
        return String::from("~/WallData/storage");
    }
    
    fn get_key_path(&self) -> String {
        return String::from("~/WallData/key");
    }
}

pub fn get_config<'a>() -> &'a Config {
    return &ConfigInternal {}
}