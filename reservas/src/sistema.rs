use std::sync::Mutex;
use std::collections::HashMap;
use crate::usuario::Usuario;
use crate::reserva::Reserva;

pub struct Sistema {
    pub reservations: Mutex<Vec<Reserva>>,
    pub clients: Mutex<HashMap<u32, Usuario>>,
    pub next_client_id: Mutex<u32>,
    pub next_reservation_id: Mutex<u32>,
}
