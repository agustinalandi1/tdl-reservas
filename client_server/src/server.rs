use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use tokio::sync::watch;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use reservas::usuario::Usuario;
use reservas::reserva::Reserva;
use reservas::sistema::Sistema;

/// Funcion que se encarga de verificar la disponibilidad de una fecha
async fn check_availability(info: web::Json<Reserva>, sistema: web::Data<Arc<Sistema>>) -> impl Responder {
    let date = &info.date;
    let integrantes = info.cant_integrantes;

    if sistema.check_availability(date, integrantes).await{
        HttpResponse::Ok().body("Date available.")
    } else {
        HttpResponse::Conflict().body("Date already reserved for that amount of guests.")
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

    if let Err(err) = sistema.load_reservations_from_csv("reservas.csv") {
        eprintln!("Error loading reservations: {}", err);
    }

    let (tx, rx) = watch::channel(false);
    let sistema_clone = sistema.clone();

    let server_stop_flag = Arc::new(AtomicBool::new(false));
    let server_stop_flag_clone = server_stop_flag.clone();

    let servidor = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(sistema_clone.clone()))
            .app_data(web::Data::new(tx.clone()))
            .route("/check", web::post().to(check_availability))  // Nueva ruta para verificar disponibilidad
            .route("/reserve", web::post().to(create_reservation)) // Nueva ruta para crear reserva
            .route("/exit", web::get().to(stop_server)) // Manejar solicitud especial
    })
    .bind("127.0.0.1:8080")?
    .run();

    // Spawn a task to listen for stop signal
    tokio::spawn(async move {
        rx.clone().changed().await.unwrap();
        // Signal received, time to shut down
        server_stop_flag_clone.store(true, Ordering::Relaxed);
        if let Err(err) = sistema.save_reservations_to_csv("reservas.csv") {
            eprintln!("Error saving reservations: {}", err);
        }
    });

    let _ = tokio::spawn(async move {
        if server_stop_flag.load(Ordering::Relaxed) {
            return Err(std::io::Error::new(std::io::ErrorKind::Interrupted, "Server stopping signal received".to_string()));
        }
        servidor.await
    })
    .await;

    Ok(())
}

// Función para detener el servidor
async fn stop_server(data: web::Data<watch::Sender<bool>>) -> HttpResponse {
    println!("Stopping server...");
    let _ = data.send(true); 
    //HttpResponse::Ok().body("Server is stopping...");
    HttpResponse::Ok().finish()
    //std::process::exit(0);
}