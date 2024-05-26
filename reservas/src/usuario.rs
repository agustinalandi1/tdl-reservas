use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Usuario {
    pub id: u32,
    pub name: String,
    pub email: String,
}
