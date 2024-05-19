use crate::reserva::Reserva;
use chrono::NaiveDate;

struct Sistema_reservas {
    reservas: Vec<Reserva>
}

fn aceptar_reserva(&self, reserva: Reserva) {
    self.reservas.push(reserva);
}

fn modificar_reserva(&self, id_reserva: u8, nueva_reserva: Reserva) {
    self.reservas[id_reserva] = nueva_reserva;
    /* IMPLEMENTAR
    fecha_inicio,
    fecha_fin,
    cantidad_huespedes,*/
}

fn cancelar_reserva(&self, id_reserva: u8) {
    self.reservas.remove(id_reserva);
}

fn obtener_habitaciones_disponibles(&self, fecha: NaiveDate) -> Vec<Habitacion> {
    let mut habitaciones_disponibles: Vec<Habitacion> = Vec::new();
    for reserva in self.reservas {
        if reserva.esta_activa(fecha) {
            habitaciones_disponibles.push(reserva.id_habitacion);
        }
    }
    habitaciones_disponibles
}
