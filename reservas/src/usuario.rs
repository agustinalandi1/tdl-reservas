use crate::habitacion::Habitacion;
use crate::sistema::SistemaReservas;

pub struct Usuario {
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

    fn solicitar_disponibilidad(&self, sistema: &SistemaReservas, fecha: &str) -> Vec<Habitacion> {
        sistema.obtener_habitaciones_disponibles(fecha)
    }
}


/*A implementar crear_reserva, editar_reserva, cancelar_reserva */