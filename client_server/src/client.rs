use reqwest::Client as HttpClient;
use std::io::{self, Write};
use reservas::{habitacion::Habitacion, usuario::Usuario};
use reservas::reserva::Reserva;
extern crate csv;
mod input_validator;
use input_validator::{DateValidator, EmailValidator, PasswordValidator, Validator};
// Add the missing import for the `validate_email` function


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
async fn menu_login_user(http_client: &HttpClient) -> Result<(Usuario), Box<dyn std::error::Error>> {
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


async fn logged_menu(http_client: &HttpClient, user: &Usuario) -> Result<(), Box<dyn std::error::Error>> {
    let mut option = String::new();
    println!("\nWelcome {} (id: #{})", user.get_email(), user.get_id());
    loop {
        //print!("{}[2J", 27 as char); // Clear the screen
        println!("1. Create a reservation");
        println!("2. Check your reservations");
        println!("3. Check availables rooms");
        println!("4. Logout current account");
        println!("Enter an option: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut option)?;

        match option.trim().parse::<i32>() {
            Ok(value) => {
                match value {
                    1 => {
                        print!("todo")
                    }
                    2 => {
                        let response = http_client.post("http://127.0.0.1:8080/get_reservations").json(&user.get_id()).send().await?;
                        
                        let list_of_reservations: Vec<Reserva> = response.json().await?;
                        
                        if list_of_reservations.len() > 0 {
                            println!("{0: <16} | {1: <10} | {2: <10} | {3: <10} | {4: <10}",
                                "Reservation ID", "Room Number", "Start Date", "End Date", "Guests");
                            for reserve in list_of_reservations.iter() {
                                println!("{0: <16} | {1: <11} | {2: <10} | {3: <9} | {4: <11}",
                                reserve.id, reserve.room_number_id, reserve.date_start, reserve.date_end, reserve.cant_integrantes);
                            }
                        }
                        else {
                            println!("No reservations found for your id!");
                        }
                    }
                    3 => {
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
                    4 => {
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

/*
/// Funcion que se encarga de leer el email del usuario
fn read_email() -> Result<String, io::Error> {
    let pattern_email = Regex::new(r"(^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$)").unwrap();
    let mut email = String::new();
    print!("Enter your email: ");
    loop {
        io::stdout().flush()?;
        io::stdin().read_line(&mut email)?;
        
        if pattern_email.is_match(&email.trim()) {
            break;
        } else {
            print!("Invalid email. Please enter a valid email: ");
            email.clear();
        }
    }
    Ok(email.trim().to_string())
}

/// Funcion que se encarga de leer la fecha de la reserva
fn read_date() -> Result<String, io::Error> {
    let pattern_dates = Regex::new(r"(\d{4})-(\d{2})\-(\d{2})").unwrap();
    let mut date = String::new();
    print!("Enter the reservation date (YYYY-MM-DD): ");
    loop {
        io::stdout().flush()?;
        io::stdin().read_line(&mut date)?;
        if pattern_dates.is_match(&date) {
            let fecha_actual = Utc::today().naive_utc();
            let groups = pattern_dates.captures(&date).unwrap();
            let year: i32 = groups.get(1).unwrap().as_str().parse().unwrap();
            let month: u32 = groups.get(2).unwrap().as_str().parse().unwrap();
            let day: u32 = groups.get(3).unwrap().as_str().parse().unwrap();
            if year == fecha_actual.year() {
                if month > fecha_actual.month() && month <=12 && day >= 1 && day <= 30{
                    break;
                } else if month == fecha_actual.month() && day >= fecha_actual.day() && day <= 30{
                    break;
                } else {
                    print!("Invalid date for actual year. Please enter a valid date (YYYY-MM-DD): ");
                    date.clear();
                }
            } else if  year == fecha_actual.year() + 1 {
                if month < fecha_actual.month() && month >= 1 && day <= 30 && day >= 1 {
                    break;
                } else if month == fecha_actual.month() && day <= fecha_actual.day() && day >= 1 {
                    break;
                } else {
                    print!("Invalid date for next year. Please enter a valid date (YYYY-MM-DD): ");
                    date.clear();
                }
            } else {
                print!("Invalid year. Please enter a valid date (YYYY-MM-DD): ");
                date.clear();
            }
        } else {
            print!("Invalid date format. Please enter a valid date (YYYY-MM-DD): ");
            date.clear();
        }
    }
    Ok(date.trim().to_string())
}

/// Funcion que se encarga de leer la cantidad de integrantes
fn read_cant_integrantes() -> Result<u8, io::Error> {
    let mut cant = String::new();
    print!("Enter the amount of guests: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut cant)?;
    match cant.trim().parse::<u8>() {
        Ok(cant) => Ok(cant),
        Err(_) => {
            eprintln!("Invalid email. Please enter a valid email: ");
            Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid input"))
        }
    }
}

/// Funcion que se encarga de leer la informacion del usuario
fn read_user_input() -> Result<(String, String, String, u8), io::Error> {
    println!("You're making a reservation. You can schedule it for a period of up to one year in advance.");
    let name = read_name()?;
    let email = read_email()?;
    let date = read_date()?;
    let integrantes = read_cant_integrantes()?;
    Ok((name, email, date, integrantes))     //no se puede poner integrantes aca, hace falta??
}

/// Funcion que se encarga de verificar la disponibilidad de la fecha
async fn check_availability(http_client: &HttpClient, reservation_check: &Reserva) -> Result<bool, reqwest::Error> {
    let res = http_client.post("http://127.0.0.1:8080/check")
        .json(reservation_check)
        .send()
        .await?;
    Ok(res.status().is_success())
}

/// Funcion que se encarga de crear la reserva
async fn create_reservation(http_client: &HttpClient, usuario: &Usuario, date: &String, integrantes: &u8) -> Result<(), reqwest::Error> {
    let res = http_client.post("http://127.0.0.1:8080/reserve")
        .json(&(usuario, date, integrantes))
        .send()
        .await?;
    println!("{}", res.text().await?);
    Ok(())
}
*/