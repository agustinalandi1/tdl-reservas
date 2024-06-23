use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::iter::Cloned;
use reservas::{habitacion::Habitacion, usuario::Usuario};
use reservas::reserva::Reserva;
extern crate csv;
mod input_validator;
use input_validator::{DateValidator, EmailValidator, PasswordValidator, Validator};
// Add the missing import for the `validate_email` function

#[derive(Serialize)]
/// Estructura de solicitud para la creación de una reserva
struct ReservationRequest {
    client_id: u32,
    room_number: u32,
    date_start: String,
    date_end: String,
    cant_integrantes: u8,
}

#[derive(Deserialize)]
/// Estructura de respuesta para la disponibilidad de una fecha
struct AvailabilityResponse {
    available: bool,
}

/// Funcion que se encarga de verificar la disponibilidad de una habitacion
async fn check_availability(http_client: &HttpClient, reservation: &Reserva) -> Result<bool, Box<dyn std::error::Error>> {
    let url = "http://127.0.0.1:8080/check";
    let request = ReservationRequest {
        client_id: reservation.client_id,
        room_number: reservation.room_number_id,
        date_start: reservation.date_start.clone(),
        date_end: reservation.date_end.clone(),
        cant_integrantes: reservation.cant_integrantes,
    };
    let response = http_client.post(url).json(&request).send().await?;
    let availability_response: AvailabilityResponse = response.json().await?;
    Ok(availability_response.available)
}

/// Funcion que se encarga de crear una reserva
async fn create_reservation(http_client: &HttpClient, user: &Usuario, room_number: u32, date_start: String, date_end: String, cant_integrantes: u8) -> Result<u32, Box<dyn std::error::Error>> {
    let url = "http://127.0.0.1:8080/reserve";
    let request = ReservationRequest {
        client_id: user.get_id(),
        room_number,
        date_start,
        date_end,
        cant_integrantes,
    };
    let response = http_client.post(url).json(&request).send().await?;
    let reservation_id: u32 = response.json().await?;
    Ok(reservation_id)
}

pub fn ask_dates_and_number_guest() -> (String, String, u8) {
    let mut date_start = String::new();
    let mut date_end = String::new();
    let mut cant_integrantes = String::new();

    let DateValidator = DateValidator;
    print!("Enter start date (YYYY-MM-DD): ");
    loop {
        io::stdout().flush();
        io::stdin().read_line(&mut date_start);
        match DateValidator.validate(&date_start) {
            Ok(_) => break,
            Err(e) => println!("{}", e),
        }
        date_start.clear();
    }
    print!("Enter end date (YYYY-MM-DD): ");
    loop {
        io::stdout().flush();
        io::stdin().read_line(&mut date_end);
        match DateValidator.validate(&date_end) {
            Ok(_) => break,
            Err(e) => println!("{}", e),
        }
        date_end.clear();
    }
    print!("Enter number of guests: ");
    loop {
        io::stdout().flush();
        io::stdin().read_line(&mut cant_integrantes);
        match cant_integrantes.trim().parse::<u8>() {
            Ok(_) => break,
            Err(_) => println!("Invalid number of guests. Please enter a valid number"),
        }
        cant_integrantes.clear();
    }
    let ucant_integrantes = cant_integrantes.trim().parse::<u8>().unwrap();

    (date_start.trim().to_owned(), date_end.trim().to_owned(), ucant_integrantes)

}
/// Funcion que se encarga de crear una reserva
pub async fn menu_create_reservation(http_client: &HttpClient, user: &Usuario) -> Result<(), Box<dyn std::error::Error>> {
    let (date_start, date_end, cant_integrantes) = ask_dates_and_number_guest();

    let d_start = date_start.clone(); 
    let d_end = date_end.clone();

    let request = http_client.post("http://127.0.0.1:8080/check").json(&(date_start, date_end, cant_integrantes)).send().await?;
    let vec_habitaciones_disponibles: Vec<Habitacion> = request.json().await?;

    if vec_habitaciones_disponibles.len() <= 0 {
        println!("No rooms available for the selected dates and number of guests.");
        return Ok(());
    }

    println!("Available rooms:");
    println!("{0: <16} | {1: <10}",
        "Room Number", "Max number of available guests"
    );

    for room in vec_habitaciones_disponibles.iter() {
        println!("{0: <16} | {1: <10}", room.id_habitacion(), room.cantidad_huespedes());
    }
    println!("Enter room number: ");
    let mut input_room = String::new();
    loop {
        io::stdin().read_line(&mut input_room)?;
        match input_room.trim().parse::<u32>() {
            Ok(value) => {
                if vec_habitaciones_disponibles.iter().any(|room| room.id_habitacion() == value) {
                    let room_number = value;
                    // client_id: u32, room_number: u32, date_start: String, date_end: String, cant_integrantes: u8
                    let request = http_client.post("http://127.0.0.1:8080/reserve").json(&(user.get_id(), room_number, d_start, d_end, cant_integrantes)).send().await?;
                    let response: u32 = request.json().await?;
                    println!("Reservation created successfully with id: {}", response.to_string());
                    break;
                } else {
                    println!("Invalid room number. Please enter a valid room number");
                }
            },
            Err(_) => println!("Invalid room number. Please enter a valid room number"),
        }
        input_room.clear();
    }
    
    Ok(())
}

/// Funcion que se encarga de eliminar una reserva
pub async fn delete_reservation(http_client: &HttpClient, user: &Usuario) -> Result<(), Box<dyn std::error::Error>> {
    let response = http_client.post("http://127.0.0.1:8080/get_reservations").json(&user.get_id()).send().await?;
    let list_of_reservations: Vec<Reserva> = response.json().await?;
    if list_of_reservations.len() > 0 {
        println!("Enter reservation ID: ");
        let mut input_id = String::new();
        let mut encontrado = false;
            io::stdin().read_line(&mut input_id)?;
            match input_id.trim().parse::<u32>() {
                Ok(value) => {
                    for reserve in list_of_reservations.iter() {
                        let (id, client_id, room_number_id, date_start, date_end, _cant_integrantes) = reserve.get_reserve_data();
                        if id == value {
                            let response = http_client.post("http://127.0.0.1:8080/delete_reservation").json(&id).send().await?;
                            let resultado: u32 = response.json().await?;
                            println!("Reservation {} deleted successfully", resultado.to_string());
                            encontrado = true;
                            break;
                        }
                    }
                    if encontrado == false {
                        println!("There's not an existent reservation with ID: {}", value);
                    }
                },
                Err(_) => println!("Invalid ID number. Please enter a valid number"),
            }
    } else {
        println!("No reservations found for your id!");
    }
    Ok(())
}

/// Funcion principal que se encarga de realizar la reserva de una habitacion
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let http_client = HttpClient::new();

    match http_client.get("http://127.0.0.1:8080").send().await {
        Ok(_) => (),
        Err(_) => {
            panic!("Server is not running!!");
        }
    }
    start_menu(&http_client).await?;

    Ok(())
}

/// Funcion que se encarga de mostrar el menú principal
async fn start_menu(http_client: &HttpClient) -> Result<(), Box<dyn std::error::Error>> {
    let mut option = String::new();
    loop {
        //print!("{}[2J", 27 as char); // Clear the screen
        println!("\nWelcome to the reservation system  made to 7531!");
        println!("1. Create a user");
        println!("2. Login existing user");
        println!("3. Exit");
        println!("Enter an option: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut option)?;

        match option.trim().parse::<i32>() {
            Ok(value) => {
                match value {
                    1 => {
                        menu_create_user(&http_client).await?;
                    }
                    2 => {
                        let login_output = menu_login_user(&http_client).await;
                        
                        match login_output {
                            Ok(value) => {
                                let user = value;
                                logged_menu(&http_client, &user).await?;
                                //Cuando llegue acá, cerró el user.
                            },
                            Err(e) => {
                                print!("{}", e);
                                continue // No se devolvio un user correcto, entonces volvemos al menú principal
                            },
                        }
                    }
                    3 => {
                        break;            
                    }
                    _ => {
                        println!("Invalid option. Please enter a valid option");
                    }
                }
            }
            Err(_) => {
                println!("Invalid option. Please enter a valid option");
            }
        };
        option.clear();
    }
    Ok(())
}

/// Funcion que se encarga de leer el email y password del usuario
async fn input_email_password() -> Result<(String, String), Box<dyn std::error::Error>> {
    let _email_validator = EmailValidator;
    let _password_validator = PasswordValidator;
    let mut email = String::new();
    let mut password = String::new();

    loop {
        println!("Enter your email: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut email)?;

        if _email_validator.validate(&email).is_ok() {
            break;
        } else {
            println!("Invalid email. Please enter a valid email");
        }
        email.clear();
    }

    loop {
        println!("Enter your password: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut password)?;

        if _password_validator.validate(&password).is_ok() {
            break;
        } else {
            println!("Invalid password. Please enter a valid password");
        }
        password.clear();
    }

    let _trimmed_email = email.trim().to_string();
    let _trimmed_password = password.trim().to_string();

    Ok((_trimmed_email, _trimmed_password))
}

/// Funcion que se encarga de crear un usuario
async fn menu_create_user(http_client: &HttpClient) -> Result<(), Box<dyn std::error::Error>> {
    let (email, password) = input_email_password().await?;
    // hasta acá mail y password envia bien.
    let result = http_client.post("http://127.0.0.1:8080/create_user").json(&(email, password)).send().await?;

    if result.status().is_success() {
        Ok(println!("User created successfully"))
    } else {
        Ok(print!("User already exists"))
    }
}

/// Funcion que se encarga de leer el nombre del usuario
async fn menu_login_user(http_client: &HttpClient) -> Result<Usuario, Box<dyn std::error::Error>> {
    let (email, password) = input_email_password().await?; 
    // las validaciones de email y password son las mismas que en create_user

    let result: reqwest::Response = http_client.post("http://127.0.0.1:8080/login").json(&(email, password)).send().await?;

    if result.status().is_success() {
        let user: Usuario = result.json().await?;
        println!("User logged in successfully");
        Ok(user)
    } else {
        Err(Box::from("User doesn't exist or incorrect password"))
    }
}

/// Funcion que se encarga de mostrar el menú de opciones para un usuario loggeado
async fn logged_menu(http_client: &HttpClient, user: &Usuario) -> Result<(), Box<dyn std::error::Error>> {
    let mut option = String::new();
    println!("\nWelcome {} (id: #{})", user.get_email(), user.get_id());
    loop {
        //print!("{}[2J", 27 as char); // Clear the screen
        println!("1. Create a reservation");
        println!("2. Check your reservations");
        println!("3. Delete a reservation");
        println!("4. Check availables rooms");
        println!("5. Logout current account");
        println!("Enter an option: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut option)?;

        match option.trim().parse::<i32>() {
            Ok(value) => {
                match value {
                    1 => {
                        menu_create_reservation(&http_client, &user).await?;
                    }
                    2 => {
                        let response = http_client.post("http://127.0.0.1:8080/get_reservations").json(&user.get_id()).send().await?;
                        
                        let list_of_reservations: Vec<Reserva> = response.json().await?;
                        
                        if list_of_reservations.len() > 0 {
                            println!("{0: <16} | {1: <10} | {2: <10} | {3: <10} | {4: <10}",
                                "Reservation ID", "Room Number", "Start Date", "End Date", "Guests");
                            for reserve in list_of_reservations.iter() {
                                let (id, client_id, room_number_id, date_start, date_end, _cant_integrantes) = reserve.get_reserve_data();
                                println!("{0: <16} | {1: <11} | {2: <10} | {3: <9} | {4: <11}",
                                id, client_id, room_number_id, date_start, date_end);
                            }
                        }
                        else {
                            println!("No reservations found for your id!");
                        }
                    }
                    3 => {
                        let response = http_client.post("http://127.0.0.1:8080/get_reservations").json(&user.get_id()).send().await?;
                        
                        let list_of_reservations: Vec<Reserva> = response.json().await?;
                        
                        if list_of_reservations.len() > 0 {
                            println!("{0: <16} | {1: <10} | {2: <10} | {3: <10} | {4: <10}",
                                "Reservation ID", "Room Number", "Start Date", "End Date", "Guests");
                            for reserve in list_of_reservations.iter() {
                                let (id, client_id, room_number_id, date_start, date_end, _cant_integrantes) = reserve.get_reserve_data();
                                println!("{0: <16} | {1: <11} | {2: <10} | {3: <9} | {4: <11}",
                                id, client_id, room_number_id, date_start, date_end);
                            }
                            delete_reservation(&http_client, &user).await?;
                        }
                        else {
                            println!("No reservations found for your id!");
                        }
                        
                    }
                    4 => {
                        let response = http_client.post("http://127.0.0.1:8080/list_all_rooms").send().await?;
                        let rooms: Vec<Habitacion> = response.json().await?;
                        println!(
                            "{0: <16} | {1: <10}",
                            "Room Number", "Max number of available guests"
                        );

                        for room in rooms.iter() {
                            println!("{0: <16} | {1: <10}", room.id_habitacion(), room.cantidad_huespedes());
                        }
                    }
                    5 => {
                        break
                    }
                    _ => {
                        println!("Invalid option. Please enter a valid option");
                    }
                }
            }
            Err(_) => {
                println!("Invalid option. Please enter a valid option");
            }
        };
        option.clear();
    }
    Ok(())
}
