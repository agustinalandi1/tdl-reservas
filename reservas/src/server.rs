use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;
use reservas::usuario::Usuario;
use reservas::reserva::Reserva;
use reservas::sistema::Sistema;

async fn check_availability(info: web::Json<Reserva>, data: web::Data<Sistema>) -> impl Responder {
    let reservations = data.reservations.lock().unwrap();
    let date = &info.date;

    for reservation in reservations.iter() {
        if &reservation.date == date {
            return HttpResponse::Conflict().body("Date already reserved");
        }
    }

    HttpResponse::Ok().body("Date available")
}

async fn create_reservation(info: web::Json<(Usuario, String)>, data: web::Data<Sistema>) -> impl Responder {
    let (usuario, date) = info.into_inner();

    let mut clients = data.clients.lock().unwrap();
    let mut next_client_id = data.next_client_id.lock().unwrap();
    let client_id = *next_client_id;
    clients.insert(client_id, Usuario { id: client_id, ..usuario });
    *next_client_id += 1;

    let mut reservations = data.reservations.lock().unwrap();
    let mut next_reservation_id = data.next_reservation_id.lock().unwrap();
    let reservation_id = *next_reservation_id;
    reservations.push(Reserva {
        id: reservation_id,
        client_id,
        date: date.clone(),
    });
    *next_reservation_id += 1;

    println!("New reservation: {:?}", reservations.last().unwrap()); // Imprimir la última reserva agregada

    HttpResponse::Ok().body(format!("Reservation confirmed with id {}", reservation_id))
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(Sistema {
        reservations: Mutex::new(Vec::new()),
        clients: Mutex::new(HashMap::new()),
        next_client_id: Mutex::new(1),
        next_reservation_id: Mutex::new(1),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .route("/check", web::post().to(check_availability))
            .route("/reserve", web::post().to(create_reservation))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
