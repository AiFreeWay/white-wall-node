pub struct Exception {
    exep_type: ExceptionTypes,
    desc: String
}
/*
impl Exception {
    
    pub fn new_synchronize_exep() -> Exception {
        return Exception { 
            exep_type: ExceptionTypes::SynchronizeErr, 
            desc: String::from("Error synchronize with blockchain") 
        }
    }
}*/

pub enum ExceptionTypes {
    Unknown = 0 
}