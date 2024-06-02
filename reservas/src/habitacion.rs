/// Representa una habitación del hotel.
#[derive(Clone)]
pub struct Habitacion {
    disponible: bool,
    cantidad_huespedes: u8,
    pub id_habitacion: u8,
}

impl Habitacion {
    /// Crea una nueva habitación.
    fn nueva(disponible: bool, cantidad_huespedes: u8, id_habitacion: u8) -> Habitacion {
        Habitacion {
            disponible,
            cantidad_huespedes,
            id_habitacion,
        }
    }

    /// Cambia la disponibilidad de la habitación.
    fn esta_disponible(&self) -> bool {
        self.disponible
    }

    /// Cambia la cantidad de huéspedes de la habitación.
    fn cantidad_huespedes(&self) -> u8 {
        self.cantidad_huespedes
    }
}