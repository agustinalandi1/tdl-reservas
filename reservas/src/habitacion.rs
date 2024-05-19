#[derive(Clone)]
pub struct Habitacion {
    disponible: bool,
    cantidad_huespedes: u8,
    pub id_habitacion: u8,
}

impl Habitacion {
    fn nueva(disponible: bool, cantidad_huespedes: u8, id_habitacion: u8) -> Habitacion {
        Habitacion {
            disponible,
            cantidad_huespedes,
            id_habitacion,
        }
    }

    fn esta_disponible(&self) -> bool {
        self.disponible
    }

    fn cantidad_huespedes(&self) -> u8 {
        self.cantidad_huespedes
    }
}