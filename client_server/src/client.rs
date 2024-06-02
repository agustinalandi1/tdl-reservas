use reqwest::Client as HttpClient;
use std::io::{self, Write};
use chrono::{Utc, Datelike};

use reservas::usuario::{self, Usuario};
use reservas::reserva::{self, Reserva};
use regex::Regex;


/// Funcion principal que se encarga de realizar la reserva de una habitacion
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let http_client = HttpClient::new();
    let (name, email, date, integrantes) = read_user_input()?;
    let usuario = usuario::Usuario::new(0, name, email);
    let reservation_check = reserva::Reserva::new(0, 0, date.clone(), integrantes);

    if check_availability(&http_client, &reservation_check).await? {
        println!("Date is available. Creating reservation...");
        create_reservation(&http_client, &usuario, &date, &integrantes).await?;
    } else {
        println!("Date is already reserved.");
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
            let año_actual = fecha_actual.year();
            if year == año_actual {
                if month >= fecha_actual.month() && month <= 12 && day >= fecha_actual.day() && day <= 30 {
                    break;
                } else {
                    print!("Invalid date. Please enter a valid date (YYYY-MM-DD): ");
                    date.clear();
                }
            } else if  year == año_actual + 1 {
                if month <= fecha_actual.month() && month >= 1 && day <= fecha_actual.day() && day >= 1 {
                    break;
                } else {
                    print!("Invalid date. Please enter a valid date (YYYY-MM-DD): ");
                    date.clear();
                }
            } else {
                print!("Invalid date. Please enter a valid date (YYYY-MM-DD): ");
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
