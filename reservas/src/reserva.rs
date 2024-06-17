use std::vec;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Representa una reserva con un id, el id del cliente y la fecha de la reserva.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reserva {
    pub id: u32,
    pub client_id: u32,
    pub room_number_id: u32,
    pub date_start: String,
    pub date_end: String,
    pub cant_integrantes: u8
}

impl Reserva {
    
    /// Crea una nueva reserva.
    pub fn new(id: u32, client_id: u32, room_number_id: u32, date_start: String, date_end: String, cant_integrantes: u8) -> Reserva {
        Reserva { id, client_id, room_number_id, date_start, date_end, cant_integrantes }
    }

    pub fn get_client_id(&self) -> u32 {
        self.client_id
    }

    pub fn get_reserve_data(&self) -> (u32, u32, u32, String, String, u8) {
        (self.id, self.client_id, self.room_number_id, self.date_start.clone(), self.date_end.clone(), self.cant_integrantes)
    }

    pub fn get_room_number_id(&self) -> u32 {
        self.room_number_id
    }

    pub fn intersection_between_dates(&self, date_start: &String, date_end: &String) -> bool {
        // dada 2 fechas, se verifica si la reserva se encuentra en el rango de fechas
        let date_start_reserva = NaiveDate::parse_from_str(&self.date_start, "%Y-%m-%d").unwrap();
        let date_end_reserva = NaiveDate::parse_from_str(&self.date_end, "%Y-%m-%d").unwrap();
        let date_start_request = NaiveDate::parse_from_str(date_start, "%Y-%m-%d").unwrap();
        let date_end_request = NaiveDate::parse_from_str(date_end, "%Y-%m-%d").unwrap();
        
        // Primer caso: La fecha de inicio está por afuera del rango, pero la fecha final está adentro del rango de la reserva.
        if (date_start_reserva >= date_start_request && (date_start_reserva <= date_end_request && date_end_reserva >= date_end_request)) ||
        // Segundo caso: La fecha de inicio y final está adentro del rango de la reserva.
           (date_start_reserva <= date_start_request && date_end_reserva >= date_end_request && date_end_request <= date_end_request) ||
        // Tercer caso: La fecha de inicio está adentro del rango de la reserva, pero la fecha final está por afuera del rango.
           ((date_start_reserva <= date_start_request && date_start_request <= date_end_reserva) && date_end_reserva >= date_start_request)      
        {
            return true;
        }
        false
    }
}