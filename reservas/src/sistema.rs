use std::fs::{File, OpenOptions};
use std::sync::Mutex;
use std::collections::HashMap;
use std::io::{self, BufReader, BufWriter};
use std::vec;
use csv::{ReaderBuilder, WriterBuilder};
use crate::usuario::Usuario;
use crate::reserva::{self, Reserva};
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

    /// Obtiene un cliente por su email.
    pub fn user_already_exists(&self, email: &String) -> bool {
        let clients = self.clients.lock().unwrap();
        clients.values().any(|client| client.email == *email)
    }

    /// Devuelve un usuario con el email y password correctos.
    pub fn user_correct_login(&self, email: &String, password: &String) -> Option<Usuario> {
        let clients = self.clients.lock().unwrap();
        let user = clients.values().find(|client| client.email == *email && client.password == *password).cloned();

        user
    }

    /// Devuelve las habitaciones.
    pub fn get_all_rooms(&self) -> Vec<Habitacion> {
        let rooms = self.rooms.lock().unwrap();
        rooms.clone()
    }

    /// Agrega un nuevo cliente al sistema.
    pub fn add_client(&self, email: String, password: String) -> u32 {
        let mut clients = self.clients.lock().unwrap();
        let mut next_client_id = self.next_client_id.lock().unwrap();
        let id: u32 = *next_client_id;
        let temp_user = Usuario::new(id, email.clone(), password.clone());
        clients.insert(id, temp_user.clone());
        *next_client_id += 1;
        self.save_client_to_csv(&temp_user);
        id
    }

    pub fn get_available_rooms(&self, date_start: &String, date_end: &String, cant_integrantes: u8) -> Vec<Habitacion> {
        let all_available_rooms = self.rooms.lock().unwrap();
        let reservations = self.reservations.lock().unwrap();

        // De las reservas, se obtienen las habitaciones que no están disponibles en las fechas dadas.
        let unavailable_rooms_ids: Vec<u32> = reservations.iter()
            .filter(|reservation| reservation.intersection_between_dates(date_start, date_end))
            .map(|reservation| reservation.room_number_id)
            .collect();

        // De todas las habitaciones, filtramos la que no están disponibles
        all_available_rooms.iter()
            .filter(|room| !unavailable_rooms_ids.contains(&room.id_habitacion()))
            .filter(|room| room.can_handle_all_guest(cant_integrantes))
            .cloned()
            .collect()
    }

    pub fn is_room_available_given_date(&self, start: &String, end :&String) -> bool {
        let reservations = self.reservations.lock().unwrap();
        reservations.iter().all(|reservation| {
            !(reservation.date_start <= *end && 
              reservation.date_end >= *start)
        })
    }
    /// Verifica si una habitación está disponible en las fechas dadas.
    pub fn is_room_available(&self, room_number: u32, date_start: &String, date_end: &String) -> bool {
        let reservations = self.reservations.lock().unwrap();
        reservations.iter().all(|reservation| {
            !(reservation.room_number_id == room_number && 
              reservation.date_start <= *date_end && 
              reservation.date_end >= *date_start)
        })
    }
    
    /// Agrega una nueva reserva al sistema.
    pub fn add_reservation(&self, client_id: u32, room_number: u32, date_start: String, date_end: String, cant_integrantes: u8) -> u32 {
        let mut reservations = self.reservations.lock().unwrap();
        let mut next_reservation_id = self.next_reservation_id.lock().unwrap();
        let id = *next_reservation_id;
        let new_reservation = Reserva::new(id, client_id, room_number, date_start, date_end, cant_integrantes);
        reservations.push(new_reservation.clone());
        *next_reservation_id += 1;
        self.save_reservation_to_csv(new_reservation);
        id
    }

    /// Verifica si una fecha está disponible. Está disponible si no hay ninguna reserva para esa fecha, o si la hay
    /// pero para habitaciones con una cantidad distinta de integrantes.
    pub async fn check_availability(&self, room_number: u32, date_start: &String, date_end: &String) -> bool {
        let reservations = self.reservations.lock().unwrap();
        reservations.iter().all(|reservation| {
            !(reservation.room_number_id == room_number && 
            reservation.date_start <= *date_end && 
            reservation.date_end >= *date_start)
        })
    }

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

    /// Guarda un cliente en un archivo CSV.
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

    fn save_reservation_to_csv(&self, reservation: Reserva) {
        let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&self.files_and_headers[1].0)
        .unwrap();
        let mut writer = csv::Writer::from_writer(file);
        let (id, client_id, room_number, date_start, date_end, number_guest) = reservation.get_reserve_data();
        writer.write_record(&[id.to_string(), client_id.to_string(), room_number.to_string(), date_start, date_end, number_guest.to_string()]).unwrap();
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
