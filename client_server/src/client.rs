use reqwest::Client as HttpClient;
use serde::Serialize;
use std::io::{self, Write};
use reservas::{habitacion::Habitacion, usuario::Usuario};
use reservas::reserva::Reserva;
extern crate csv;
mod input_validator;
use input_validator::{DateValidator, EmailValidator, PasswordValidator, Validator};

#[derive(Serialize)]
/// Estructura de solicitud para la creación de una reserva
struct ReservationRequest {
    client_id: u32,
    room_number: u32,
    date_start: String,
    date_end: String,
    cant_integrantes: u8,
}

/// Funcion que se encarga de preguntar las fechas y la cantidad de huespedes, y validarlo
pub fn ask_dates_and_number_guest() -> (String, String, u8) {
    let mut date_start = String::new();
    let mut date_end = String::new();
    let mut cant_integrantes = String::new();

    let DateValidator = DateValidator;
    print!("Enter start date (YYYY-MM-DD): ");
    loop {
        let _ = io::stdout().flush();
        let _ = io::stdin().read_line(&mut date_start);
        match DateValidator.validate(&date_start) {
            Ok(_) => break,
            Err(e) => println!("{}", e),
        }
        date_start.clear();
    }
    print!("Enter end date (YYYY-MM-DD): ");
    loop {
        let _ = io::stdout().flush();
        let _ = io::stdin().read_line(&mut date_end);
        match DateValidator.validate(&date_end) {
            Ok(_) => break,
            Err(e) => println!("{}", e),
        }
        date_end.clear();
    }
    print!("Enter number of guests: ");
    loop {
        let _ = io::stdout().flush();
        let _ = io::stdin().read_line(&mut cant_integrantes);
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
        println!("Enter reservation ID (in case you don't want to delete any reservation, enter an incorrect ID): ");
        let mut input_id = String::new();
        let mut encontrado = false;
            io::stdin().read_line(&mut input_id)?;
            match input_id.trim().parse::<u32>() {
                Ok(value) => {
                    let response = http_client.post("http://127.0.0.1:8080/get_reservations").json(&user.get_id()).send().await?;
                    let list_of_reservations: Vec<Reserva> = response.json().await?;
                    for reserve in list_of_reservations.iter() {
                        let (id, _client_id, _room_number_id, _date_start, _date_end, _cant_integrantes) = reserve.get_reserve_data();
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
    Ok(())
}

/// Funcion que se encarga de modificar una reserva
pub async fn modify_reservation(http_client: &HttpClient, user: &Usuario) -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter reservation ID: ");
    let mut input_id = String::new();
    let mut encontrado = false;
        io::stdin().read_line(&mut input_id)?;
        match input_id.trim().parse::<u32>() {
            Ok(value) => {
                let response = http_client.post("http://127.0.0.1:8080/get_reservations").json(&user.get_id()).send().await?;
                let list_of_reservations: Vec<Reserva> = response.json().await?;
                for reserve in list_of_reservations.iter() {
                    let (id, _client_id, room_number_id, date_start, date_end, _cant_integrantes) = reserve.get_reserve_data();
                    let d_start = date_start.clone(); 
                    let d_end = date_end.clone();
                    if id == value {
                        let mut option = String::new();
                        println!("What do you want to modify?");
                        println!("1. Number of guests");
                        println!("2. Start date");
                        println!("3. End date");
                        println!("Enter an option: ");
                        io::stdout().flush()?;
                        io::stdin().read_line(&mut option)?;

                        match option.trim().parse::<i32>() {
                            Ok(value) => {
                                match value {
                                    1 => {
                                        println!("Enter new number of guests: ");
                                        let mut new_guests_str = String::new();
                                        io::stdin().read_line(&mut new_guests_str)?;
                                        match new_guests_str.trim().parse::<u8>() {
                                            Ok(new_guests) => {
                                                let response = http_client.post("http://127.0.0.1:8080/list_all_rooms").send().await?;
                                                let rooms: Vec<Habitacion> = response.json().await?;
                                                if rooms.iter().any(|hab| hab.id_habitacion() == room_number_id && hab.cantidad_huespedes() >= new_guests) {
                                                    let response = http_client.post("http://127.0.0.1:8080/modify_reservation").json(&(d_start, d_end, new_guests, id)).send().await?;
                                                    let resultado: u32 = response.json().await?;
                                                    println!("Number of guests modified successfully for reservation {}", resultado.to_string());
                                                    encontrado = true;
                                                    break;
                                                } else {
                                                    println!("The room doesn't have the capacity for that amount of guests. Please delete this reservation and create a new one.");
                                                    break;
                                                }
                                            },
                                            Err(_) => println!("Invalid number. Please enter a valid number"),
                                        }
                                        
                                    }
                                    2 => {
                                        println!("Enter new start date (YYYY-MM-DD): ");
                                        let mut new_enter_date = String::new();
                                        io::stdin().read_line(&mut new_enter_date)?;
                                        let new_enter_date = new_enter_date.trim().to_owned(); 
                                        let new_d_start = new_enter_date.clone();  
                                        let _response = http_client.post("http://127.0.0.1:8080/delete_reservation").json(&id).send().await?;
                                        let request = http_client.post("http://127.0.0.1:8080/check").json(&(new_enter_date, date_end, _cant_integrantes)).send().await?;
                                        let vec_habitaciones_disponibles: Vec<Habitacion> = request.json().await?;
                                        if vec_habitaciones_disponibles.iter().any(|hab| hab.id_habitacion() == room_number_id) {
                                            let request = http_client.post("http://127.0.0.1:8080/reserve").json(&(user.get_id(), room_number_id, new_d_start, d_end, _cant_integrantes)).send().await?;
                                            let response: u32 = request.json().await?;
                                            println!("Start date modified successfully for reservation {}", response.to_string());
                                            encontrado = true;
                                            break;
                                        } else {
                                            let _request = http_client.post("http://127.0.0.1:8080/reserve").json(&(user.get_id(), room_number_id, d_start, d_end, _cant_integrantes)).send().await?;
                                            println!("The room is not available for that date. Please delete this reservation and create a new one.");
                                            break;
                                        }
                                    }
                                    3 => {
                                        println!("Enter new end date (YYYY-MM-DD): ");
                                        let mut new_end_date = String::new();
                                        io::stdin().read_line(&mut new_end_date)?;
                                        let new_end_date = new_end_date.trim().to_owned(); 
                                        let new_d_end = new_end_date.clone();  
                                        let _response = http_client.post("http://127.0.0.1:8080/delete_reservation").json(&id).send().await?;
                                        let request = http_client.post("http://127.0.0.1:8080/check").json(&(date_start, new_end_date, _cant_integrantes)).send().await?;
                                        let vec_habitaciones_disponibles: Vec<Habitacion> = request.json().await?;
                                        if vec_habitaciones_disponibles.iter().any(|hab| hab.id_habitacion() == room_number_id) {
                                            let request = http_client.post("http://127.0.0.1:8080/reserve").json(&(user.get_id(), room_number_id, d_start, new_d_end, _cant_integrantes)).send().await?;
                                            let response: u32 = request.json().await?;
                                            println!("End date modified successfully for reservation {}", response.to_string());
                                            encontrado = true;
                                            break;
                                        } else {
                                            let _request = http_client.post("http://127.0.0.1:8080/reserve").json(&(user.get_id(), room_number_id, d_start, d_end, _cant_integrantes)).send().await?;
                                            println!("The room is not available for that date. Please delete this reservation and create a new one.");
                                            break;
                                        }
                                    }
                                    _ => {
                                        println!("Invalid option. Please enter a valid option");
                                    }
                                }                        
                            },
                            Err(_) => println!("Invalid option. Please enter a valid option"),
                        }
                    }
                }
                if encontrado == false {
                    println!("There's not an existent reservation with ID: {}", value);
                }
            },
            Err(_) => println!("Invalid ID number. Please enter a valid number"),
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
        println!("4. Modify a reservation");
        println!("5. Check availables rooms");
        println!("6. Logout current account");
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
                                let (id, _client_id, room_number_id, date_start, date_end, _cant_integrantes) = reserve.get_reserve_data();
                                println!("{0: <16} | {1: <11} | {2: <10} | {3: <9} | {4: <11}",
                                id, room_number_id, date_start, date_end, _cant_integrantes);
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
                                let (id, _client_id, room_number_id, date_start, date_end, _cant_integrantes) = reserve.get_reserve_data();
                                println!("{0: <16} | {1: <11} | {2: <10} | {3: <9} | {4: <11}",
                                id, room_number_id, date_start, date_end, _cant_integrantes);
                            }
                            delete_reservation(&http_client, &user).await?;
                        }
                        else {
                            println!("No reservations found for your id!");
                        }
                        
                    }
                    4 => {
                        let response = http_client.post("http://127.0.0.1:8080/get_reservations").json(&user.get_id()).send().await?;
                        
                        let list_of_reservations: Vec<Reserva> = response.json().await?;
                        
                        if list_of_reservations.len() > 0 {
                            println!("{0: <16} | {1: <10} | {2: <10} | {3: <10} | {4: <10}",
                                "Reservation ID", "Room Number", "Start Date", "End Date", "Guests");
                            for reserve in list_of_reservations.iter() {
                                let (id, _client_id, room_number_id, date_start, date_end, _cant_integrantes) = reserve.get_reserve_data();
                                println!("{0: <16} | {1: <11} | {2: <10} | {3: <9} | {4: <11}",
                                id, room_number_id, date_start, date_end, _cant_integrantes);
                            }
                            modify_reservation(&http_client, &user).await?;
                        }
                        else {
                            println!("No reservations found for your id!");
                        }
                        
                    }
                    5 => {
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
                    6 => {
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
