use std::fs::File;
use std::sync::Mutex;
use std::collections::HashMap;
use std::io::{self, BufReader, BufWriter};
use csv::{ReaderBuilder, WriterBuilder};
use crate::usuario::Usuario;
use crate::reserva::Reserva;
use crate::habitacion::Habitacion;

/// Representa el sistema de reservas con una lista de reservas, una lista de clientes, el id del siguiente cliente
///  y el id de la siguiente reserva.
#[derive(Debug)]
pub struct Sistema {
    pub reservations: Mutex<Vec<Reserva>>,
    pub clients: Mutex<HashMap<u32, Usuario>>,
    pub next_client_id: Mutex<u32>,
    pub next_reservation_id: Mutex<u32>,
}

impl Sistema {
    /// Crea un nuevo sistema de reservas.
    pub fn new() -> Sistema {
        Sistema {
            reservations: Mutex::new(Vec::new()),
            clients: Mutex::new(HashMap::new()),
            next_client_id: Mutex::new(1),
            next_reservation_id: Mutex::new(1),
        }
    }

    /// Agrega un nuevo cliente al sistema.
    pub fn add_client(&self, name: String, email: String) -> u32 {
        let mut clients = self.clients.lock().unwrap();
        let mut next_client_id = self.next_client_id.lock().unwrap();
        let id = *next_client_id;
        clients.insert(id, Usuario::new(id, name, email));
        *next_client_id += 1;
        id
    }

    /// Agrega una nueva reserva al sistema.
    pub fn add_reservation(&self, client_id: u32, date: String, cant_integrantes: u8) -> u32 {
        let mut reservations = self.reservations.lock().unwrap();
        let mut next_reservation_id = self.next_reservation_id.lock().unwrap();
        let id = *next_reservation_id;
        let hab = Habitacion::nueva(true, cant_integrantes, id); 
        reservations.push(Reserva::new(id, client_id, date, cant_integrantes));
        *next_reservation_id += 1;
        id
    }

    /// Verifica si una fecha está disponible. Está disponible si no hay ninguna reserva para esa fecha, o si la hay
    /// pero para habitaciones con una cantidad distinta de integrantes.
    pub async fn check_availability(&self, date: &String, cant_integrantes: u8) -> bool {
        let reservations = self.reservations.lock().unwrap();
        reservations.iter().all(|reservation| {
            reservation.date != *date || 
            reservation.cant_integrantes != cant_integrantes
        })
    }

    /// Obtiene una reserva por su id.
    pub fn get_reservation(&self, id: u32) -> Option<Reserva> {
        let reservations = self.reservations.lock().unwrap();
        reservations.iter().find(|reservation| reservation.id == id).cloned()
    }

    /// Guarda las reservas en un archivo CSV.
    pub fn save_reservations_to_csv(&self, filename: &str) -> Result<(), io::Error> {
        let reservations = self.reservations.lock().unwrap();
        let file = File::create(filename)?;
        let writer = BufWriter::new(file);
        let mut csv_writer = WriterBuilder::new().from_writer(writer);

        for reservation in reservations.iter() {
            csv_writer.serialize(reservation)?;
        }

        csv_writer.flush()?;
        Ok(())
    }

    /// Carga las reservas desde un archivo CSV.
    pub fn load_reservations_from_csv(&self, filename: &str) -> Result<(), io::Error> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let mut csv_reader = ReaderBuilder::new().from_reader(reader);

        for result in csv_reader.deserialize::<(u32, u32, String, u8)>() {
            let (id, client_id, date, cant_integrantes): (u32, u32, String, u8) = result?;
            let mut reservations = self.reservations.lock().unwrap();
            reservations.push(Reserva::new(id, client_id, date, cant_integrantes));
        }

        Ok(())
    }
}
