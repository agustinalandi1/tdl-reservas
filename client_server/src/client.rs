use reqwest::Client as HttpClient;
use std::io::{self, Write};
use chrono::{Utc, Datelike};

use reservas::usuario::{self, Usuario};
use reservas::reserva::{self, Reserva};
use regex::Regex;

extern crate csv;

use std::error::Error;
//use std::fs::File;
use csv::Reader;
use std::fs::{self, File};


/// Funcion principal que se encarga de realizar la reserva de una habitacion
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let http_client = HttpClient::new();

    loop {
        let (name, email, date, integrantes) = read_user_input()?;
        let usuario = usuario::Usuario::new(0, name, email);
        let reservation_check = reserva::Reserva::new(0, 0, date.clone(), integrantes);

        if check_availability(&http_client, &reservation_check).await? {
            println!("Date is available. Creating reservation...");
            create_reservation(&http_client, &usuario, &date, &integrantes).await?;
        } else {
            println!("Date is already reserved.");
        }

        let mut input = String::new();                                  //VER BIEN COMO QUEREMOS QUE SEA EL DIALOGO CON USER, ESTO PRIMERO.... ETC
        println!("Do you want to perform any other task? (enter the correct index): ");
        println!("1. Search for reservation");
        println!("2. Delete reservation (enter reservation id)");
        println!("3. Modify reservation (enter reservation id)");
        println!("4. Make another reservation");
        println!("5. Terminate session");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        let input = input.trim().to_lowercase();
        match input.as_str() {                              //MODULARIZAR
            "1" => {
                println!("Enter reservation id: ");
                let mut id = String::new();
                io::stdout().flush()?;
                io::stdin().read_line(&mut id)?;
                let id_int = match id.trim().parse::<u8>() {
                    Ok(value) => value,
                    Err(_) => {
                        println!("Invalid input for reservation id");
                        continue;
                    }
                };
                search_reservation(id_int);
            }
            "2" => {                                //BORRAR EN SERVER?
                println!("Enter reservation id: ");
                let mut id = String::new();
                io::stdout().flush()?;
                io::stdin().read_line(&mut id)?;
                let id_int = match id.trim().parse::<u8>() {
                    Ok(value) => value,
                    Err(_) => {
                        println!("Invalid input for reservation id");
                        continue;
                    }
                };
                delete_reservation(id_int);
            }
            "3" => continue, //TERMINAR DE HACER MODIFY
            "4" => continue,
            "5" => {
                let _ = http_client.get("http://127.0.0.1:8080/exit").send().await;
                //break;
            }
            _ => println!("Invalid input. Please enter a correct number."),
        }
        break;  //VER DONDE PONER EL BREAK
    }

    Ok(())
}

//Funcion que busca una reserva ya creada
fn search_reservation(id_input: u8) -> Result<(), Box<dyn Error>> {
    let file = File::open("reservas.csv")?;
    let mut rdr = Reader::from_reader(file);
    let mut found = false;

    for result in rdr.records() {
        let record = result?;
        if let Some(id) = record.get(0) {
            if let Ok(id) = id.parse::<u8>() {
                if id == id_input {
                    found = true;
                    for field in record.iter() {
                        print!("{}, ", field);      //imprimir parte por parte con su identificacoin
                    }
                }
            }
        }
    }
    if !found {
        return Err(Box::new(io::Error::new(io::ErrorKind::NotFound, "No existe ninguna reserva con ese id")));
    }

    Ok(())
}

// Funcion que elimina reserva pasada por parametro
fn delete_reservation(id_input: u8) -> Result<(), Box<dyn Error>> {
    let file = File::open("reservas.csv")?;
    let mut rdr = Reader::from_reader(file);
    let mut writer = csv::Writer::from_path("temp.csv")?;

    let mut found = false;

    for result in rdr.records() {
        let record = result?;
        if let Some(id) = record.get(0) {
            if let Ok(id) = id.parse::<u8>() {
                if id == id_input {
                    found = true;
                    continue;
                }
            }
        }
        writer.write_record(&record)?;
    }
    drop(writer);
    drop(rdr);
    fs::rename("temp.csv", "reservas.csv")?;

    if found {
        println!("Reservation deleted successfully!");
    } else {
        println!("There's no existing reservation with that id");
    }

    Ok(())
}

/// Funcion que se encarga de leer el nombre del usuario
fn read_name() -> Result<String, io::Error> {
    let mut name = String::new();
    print!("Enter your name: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut name)?;
    Ok(name.trim().to_string())
}

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
