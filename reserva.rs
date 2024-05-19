use chrono::NaiveDate;

struct Reserva {
    id_usuario: u8,
    fecha_inicio: NaiveDate,
    fecha_fin: NaiveDate,
    id_habitacion: u8,
    cantidad_huespedes: u8,
}

impl Reserva {
    fn nueva(id_usuario: u8, fecha_inicio: NaiveDate, fecha_fin: NaiveDate, id_habitacion: u8, cantidad_huespedes: u8) -> Reserva {
        Reserva {
            id_usuario,
            fecha_inicio,
            fecha_fin,
            id_habitacion,
            cantidad_huespedes,
        }
    }

    fn esta_activa(&self, fecha: NaiveDate) -> bool {
        fecha >= self.fecha_inicio && fecha <= self.fecha_fin
    }
}