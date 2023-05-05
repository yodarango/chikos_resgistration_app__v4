use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Registrant {
    pub id: u64,
    pub name: String,
}