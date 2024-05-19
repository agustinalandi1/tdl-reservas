use crate::reserva::Reserva;
use crate::habitacion::Habitacion;

pub struct SistemaReservas {
    reservas: Vec<Reserva>,
    habitaciones: Vec<Habitacion>,
}

impl SistemaReservas {

    pub fn new() -> SistemaReservas {
        SistemaReservas {
            reservas: Vec::new(),
            habitaciones: Vec::new(),
        }
    }

    fn aceptar_reserva(&mut self, reserva: Reserva) {
        self.reservas.push(reserva);
    }
    
    fn modificar_reserva(&mut self, id_reserva: usize, nueva_reserva: Reserva) {
        if id_reserva < self.reservas.len() {
            self.reservas[id_reserva] = nueva_reserva;
        }
        /* IMPLEMENTAR
        fecha_inicio,
        fecha_fin,
        cantidad_huespedes,*/
    }
    
    fn cancelar_reserva(&mut self, id_reserva: usize) {
        if id_reserva < self.reservas.len() {
            self.reservas.remove(id_reserva);
        }
    }
    
    pub fn obtener_habitaciones_disponibles(&self, fecha: &str) -> Vec<Habitacion> {
        let mut habitaciones_disponibles: Vec<Habitacion> = Vec::new();
        for habitacion in &self.habitaciones {
            let mut disponible = true;
            for reserva in &self.reservas {
                if reserva.id_habitacion == habitacion.id_habitacion && reserva.esta_activa(fecha.to_string()) {
                    disponible = false;
                    break;
                }
            }
            if disponible {
                habitaciones_disponibles.push(habitacion.clone());
            }
        }
        habitaciones_disponibles
    }
    

}
