use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

use reservas::usuario::Usuario;
use reservas::reserva::Reserva;



#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let http_client = HttpClient::new();
    let (name, email, date) = read_user_input()?;
    let usuario = Usuario { id: 0, name, email };
    let reservation_check = Reserva { id: 0, client_id: 0, date: date.clone() };

    if check_availability(&http_client, &reservation_check).await? {
        println!("Date is available. Creating reservation...");
        create_reservation(&http_client, &usuario, &date).await?;
    } else {
        println!("Date is already reserved.");
    }

    Ok(())
}

fn read_user_input() -> Result<(String, String, String), io::Error> {
    let mut name = String::new();
    let mut email = String::new();
    let mut date = String::new();

    print!("Enter your name: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut name)?;

    print!("Enter your email: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut email)?;

    print!("Enter the reservation date (YYYY-MM-DD): ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut date)?;

    Ok((name.trim().to_string(), email.trim().to_string(), date.trim().to_string()))
}

async fn check_availability(http_client: &HttpClient, reservation_check: &Reserva) -> Result<bool, reqwest::Error> {
    let res = http_client.post("http://127.0.0.1:8080/check")
        .json(reservation_check)
        .send()
        .await?;
    Ok(res.status().is_success())
}

async fn create_reservation(http_client: &HttpClient, usuario: &Usuario, date: &String) -> Result<(), reqwest::Error> {
    let res = http_client.post("http://127.0.0.1:8080/reserve")
        .json(&(usuario, date))
        .send()
        .await?;
    println!("{}", res.text().await?);
    Ok(())
}
