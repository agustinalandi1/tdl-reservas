
pub struct Reserva {
    id_usuario: u8,
    fecha_inicio: String,
    fecha_fin: String,
    pub id_habitacion: u8,
    cantidad_huespedes: u8,
}

impl Reserva {
    fn nueva(id_usuario: u8, fecha_inicio: String, fecha_fin: String, id_habitacion: u8, cantidad_huespedes: u8) -> Reserva {
        Reserva {
            id_usuario,
            fecha_inicio,
            fecha_fin,
            id_habitacion,
            cantidad_huespedes,
        }
    }

    pub fn esta_activa(&self, fecha: String) -> bool {
        fecha >= self.fecha_inicio && fecha <= self.fecha_fin
    }
}