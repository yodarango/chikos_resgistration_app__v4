use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Registrant {
    pub id: u64,
    pub name: String,
}