use serde::{Deserialize, Serialize};

/// Representa una reserva con un id, el id del cliente y la fecha de la reserva.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reserva {
    pub id: u32,
    pub client_id: u32,
    pub room_number_id: u32,
    pub date_start: String,
    pub date_end: String,
    pub cant_integrantes: u8
}

impl Reserva {
    
    /// Crea una nueva reserva.
    pub fn new(id: u32, client_id: u32, room_number_id: u32, date_start: String, date_end: String, cant_integrantes: u8) -> Reserva {
        Reserva { id, client_id, room_number_id, date_start, date_end, cant_integrantes }
    }

    /// Obtiene el id del usuario.
    pub fn get_client_id(&self) -> u32 {
        self.client_id
    }
}
