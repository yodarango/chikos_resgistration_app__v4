use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Registrant {
    pub first_name: String,
    pub last_name: String,
    pub id: Option<u64>,
    pub gender: u8,
    pub age: u8,
}