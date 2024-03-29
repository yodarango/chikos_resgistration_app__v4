use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Registrant {
    pub  created_date: Option<String>,
    pub guardian: Option<Guardian>,
    pub first_name: String,
    pub last_name: String,
    pub id: Option<u64>,
    pub gender: u8,
    pub age: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Guardian {
    pub first_name: String,
    pub last_name: String,
    pub phone_number: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response {
    pub message: String,
    pub status: u16,    
    pub data: Option<Registrant>,
}