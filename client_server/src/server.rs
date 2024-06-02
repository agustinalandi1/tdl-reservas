use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::sync::Arc;
use reservas::usuario::Usuario;
use reservas::reserva::Reserva;
use reservas::sistema::Sistema;
use reservas::habitacion::{self, Habitacion};

/// Funcion que se encarga de verificar la disponibilidad de una fecha
async fn check_availability(info: web::Json<Reserva>, sistema: web::Data<Arc<Sistema>>) -> impl Responder {
    let date = &info.date;

    if sistema.check_availability(date).await{
        HttpResponse::Ok().body("Date available")
    } else {
        HttpResponse::Conflict().body("Date already reserved")
    }
}

/// Funcion que se encarga de crear una reserva
async fn create_reservation(info: web::Json<(Usuario, String, u8)>, sistema: web::Data<Arc<Sistema>>) -> impl Responder {
    let (usuario, date, cantidas_integrantes) = info.into_inner();
    let client_id = sistema.add_client(usuario.name.clone(), usuario.email.clone());
    let reservation_id = sistema.add_reservation(client_id, date.clone(), cantidas_integrantes);

    println!("New reservation: {:?}", sistema.get_reservation(reservation_id).unwrap());

    HttpResponse::Ok().body(format!("Reservation confirmed with id {}", reservation_id))
}

/// Funcion principal que se encarga de iniciar el servidor
#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let sistema = Arc::new(Sistema::new());

    HttpServer::new(move || {
        App::new()
            .data(sistema.clone())
            .route("/check", web::post().to(check_availability))
            .route("/reserve", web::post().to(create_reservation))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

