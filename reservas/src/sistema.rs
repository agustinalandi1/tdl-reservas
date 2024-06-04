use std::fs::{File, OpenOptions};
use std::sync::Mutex;
use std::collections::HashMap;
use std::io::{self, BufReader, BufWriter};
use std::vec;
use csv::{ReaderBuilder, WriterBuilder};
use crate::usuario::Usuario;
use crate::reserva::Reserva;
use crate::habitacion::Habitacion;
use std::path::Path;

/// Representa el sistema de reservas con una lista de reservas, una lista de clientes, el id del siguiente cliente
///  y el id de la siguiente reserva.
#[derive(Debug)]
pub struct Sistema {
    reservations: Mutex<Vec<Reserva>>,
    clients: Mutex<HashMap<u32, Usuario>>,
    rooms: Mutex<Vec<Habitacion>>,
    next_client_id: Mutex<u32>,
    next_reservation_id: Mutex<u32>,
    files_and_headers: Vec<(String, Vec<&'static str>)>,
}

impl Sistema {
    /// Crea un nuevo sistema de reservas.
    pub fn new() -> Sistema {
        let file_path =  std::env::current_dir().unwrap().display().to_string();
        let file_and_headers = vec![
            // directorio                                               cabezeras del CSV
            (format!("{}/storage/users.csv", file_path), vec!["id_user", "email", "password"]),
            (format!("{}/storage/reservations.csv", file_path), vec!["id_reservation", "id_user", "room_number", "date_start", "date_end", "number_of_guests"]),
            (format!("{}/storage/rooms.csv", file_path), vec!["room_number", "max_number_of_guests"])
        ];

        let sys = Sistema {
            reservations: Mutex::new(Vec::new()),
            clients: Mutex::new(HashMap::new()),
            rooms: Mutex::new(Vec::new()),
            next_client_id: Mutex::new(1),
            next_reservation_id: Mutex::new(1),
            files_and_headers: file_and_headers,
        };

        let mut file_not_found: Vec<String> = vec![];
                
        // Si no existen los archivos, creamos con su respectiva cabezera.
        for data in sys.files_and_headers.clone() {
            let file_path = data.0;
            let headers = data.1;
            if !Path::new(&file_path).exists() {
                let mut writer = csv::Writer::from_path(&file_path).unwrap();
                writer.write_record(&headers).unwrap();
                writer.flush().unwrap();

                file_not_found.push(file_path);
            }
        }

        if file_not_found.len() > 0  {
            print!("The listed files were not found and have been created: \n");
            file_not_found.iter().for_each(|file| print!("{} ", file));
            panic!("Stopping server, restart server please!");
        }
        sys
    }

    pub fn user_already_exists(&self, email: &String) -> bool {
        let clients = self.clients.lock().unwrap();
        clients.values().any(|client| client.email == *email)
    }

    pub fn user_correct_login(&self, email: &String, password: &String) -> Option<Usuario> {
        let clients = self.clients.lock().unwrap();
        let user = clients.values().find(|client| client.email == *email && client.password == *password).cloned();

        user
    }

    pub fn get_all_rooms(&self) -> Vec<Habitacion> {
        let rooms = self.rooms.lock().unwrap();
        rooms.clone()
    }
    /// Agrega un nuevo cliente al sistema.
    pub fn add_client(&self, email: String, password: String) -> u32 {
        let mut clients = self.clients.lock().unwrap();
        let mut next_client_id = self.next_client_id.lock().unwrap();
        let id: u32 = *next_client_id;
        let tempUser = Usuario::new(id, email.clone(), password.clone());
        clients.insert(id, tempUser.clone());
        *next_client_id += 1;
        self.save_client_to_csv(&tempUser);
        id
    }
/*
    /// Agrega una nueva reserva al sistema.
    pub fn add_reservation(&self, client_id: u32, date: String, cant_integrantes: u8) -> u32 {
        let mut reservations = self.reservations.lock().unwrap();
        let mut next_reservation_id = self.next_reservation_id.lock().unwrap();
        let id = *next_reservation_id;
        let hab = Habitacion::nueva( id, cant_integrantes); 
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
    }*/

    /// Obtiene una reserva por su id.
    pub fn get_reservation_by_client_id(&self, client_id: u32) -> Vec<Reserva> {
        let reservations = self.reservations.lock().unwrap();
        reservations.iter()
            .filter(|reservation| reservation.get_client_id() == client_id)
            .cloned()  // clone each item
            .collect() // collect into a Vec<Reserva>
        
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
    fn save_client_to_csv(&self, client: &Usuario) {
        let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&self.files_and_headers[0].0)
        .unwrap();
        let mut writer = csv::Writer::from_writer(file);
        writer.write_record(&[client.get_id().to_string(), client.get_email().clone(), client.get_password().clone()]).unwrap();
        writer.flush().unwrap();
    }
    /// Carga las reservas desde un archivo CSV.
    pub fn load_vital_data(&self) -> Result<(), io::Error> {
        // esto estoy 100% seguro que se puede poner mas lindo.
        let mut file_path = &self.files_and_headers[0].0;
        let mut file = File::open(file_path)?;
        let mut reader = BufReader::new(file);
        let mut csv_reader = ReaderBuilder::new().from_reader(reader);

        for result in csv_reader.deserialize::<(u32, String, String)>() {
            let (id, email, password): (u32, String, String) = result?;
            let mut clients = self.clients.lock().unwrap();
            clients.insert(id, Usuario::new(id, email, password));
            *self.next_client_id.lock().unwrap() = id + 1; // cada vez que se carga un id, se asigna el id del actual + 1 y ahí es donde va a quedar parado.
        }
        // carga de reservas
        file_path = &self.files_and_headers[1].0;
        file = File::open(file_path)?;
        reader = BufReader::new(file);
        csv_reader = ReaderBuilder::new().from_reader(reader);

        for result in csv_reader.deserialize::<(u32, u32, u32, String, String, u8)>() {
            let (reserve_id, client_id, room_id, date_start, date_end, number_guest): (u32, u32, u32, String, String, u8) = result?;
            let mut reservations = self.reservations.lock().unwrap();
            reservations.push(Reserva::new(reserve_id, client_id, room_id, date_start, date_end, number_guest));
            *self.next_reservation_id.lock().unwrap() = reserve_id + 1;
        }

        // carga de habitaciones
        file_path = &self.files_and_headers[2].0;
        file = File::open(file_path)?;
        reader = BufReader::new(file);
        csv_reader = ReaderBuilder::new().from_reader(reader);

        for result in csv_reader.deserialize::<(u32, u8)>() {
            let (room_number, room_guests): (u32, u8) = result?;
            let mut vec_rooms: std::sync::MutexGuard<Vec<Habitacion>> = self.rooms.lock().unwrap();
            vec_rooms.push(Habitacion::nueva(room_number, room_guests));
        }
        Ok(())
    }
}
