use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Reserva {
    pub id: u32,
    pub client_id: u32,
    pub date: String,
}
