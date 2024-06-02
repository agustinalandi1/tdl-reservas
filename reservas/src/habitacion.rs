/// Representa una habitación del hotel.
#[derive(Clone)]
pub struct Habitacion {
    disponible: bool,
    cantidad_huespedes: u8,
    pub id_habitacion: u32,     //lo cambie de u8 a u32 para probar en add_reservation (sistemas)
}

impl Habitacion {
    /// Crea una nueva habitación.
    pub fn nueva(disponible: bool, cantidad_huespedes: u8, id_habitacion: u32) -> Habitacion {
        Habitacion {
            disponible,
            cantidad_huespedes,
            id_habitacion,
        }
    }

    /// Cambia la disponibilidad de la habitación.
    pub fn esta_disponible(&self) -> bool {
        self.disponible
    }

    /// Cambia la cantidad de huéspedes de la habitación.
    pub fn cantidad_huespedes(&self) -> u8 {
        self.cantidad_huespedes
    }
}