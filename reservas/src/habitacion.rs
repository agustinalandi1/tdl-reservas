use serde::{Deserialize, Serialize};

/// Representa una habitación del hotel.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Habitacion {
    cantidad_huespedes: u8,
    pub id_habitacion: u32,     //lo cambie de u8 a u32 para probar en add_reservation (sistemas)
}

impl Habitacion {
    /// Crea una nueva habitación.
    pub fn nueva(id_habitacion: u32, cantidad_huespedes: u8) -> Habitacion {
        Habitacion {
            cantidad_huespedes,
            id_habitacion,
        }
    }

    /// Cambia la cantidad de huéspedes de la habitación.
    pub fn cantidad_huespedes(&self) -> u8 {
        self.cantidad_huespedes
    }

    /// Obtiene el id de la habitación.
    pub fn id_habitacion(&self) -> u32 {
        self.id_habitacion
    }

    pub fn can_handle_all_guest(&self, guests: u8) -> bool {
        self.cantidad_huespedes >= guests
    }
}