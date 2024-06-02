use serde::{Deserialize, Serialize};

/// Representa una reserva con un id, el id del cliente y la fecha de la reserva.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reserva {
    pub id: u32,
    pub client_id: u32,
    pub date: String,
    pub cant_integrantes: u8
}

impl Reserva {

    /// Crea una nueva reserva.
    pub fn new(id: u32, client_id: u32, date: String, cant_integrantes: u8) -> Reserva {
        Reserva { id, client_id, date, cant_integrantes }
    }
}
