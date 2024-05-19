use chrono::NaiveDate;

struct Usuario {
    tiene_reserva: bool,
    nombre: String,
    mail: String,
}

impl Usuario {
    fn nuevo_usuario(tiene_reserva: bool, nombre: String, mail: String) -> Usuario {
        Usuario {
            tiene_reserva,
            nombre,
            mail,
        }
    }
}

fn solicitar_disponibilidad(fecha: NaiveDate) -> Vec<Habitacion> {
    let mut habitaciones_disponibles: Vec<Habitacion> = Vec::new();
    for reserva in self.reservas {
        if reserva.esta_activa(fecha) {
            habitaciones_disponibles.push(reserva.id_habitacion);
        }
    }
    habitaciones_disponibles
}

/*A implementar crear_reserva, editar_reserva, cancelar_reserva */