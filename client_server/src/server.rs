use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use tokio::sync::watch;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use reservas::sistema::Sistema;

#[derive(Deserialize)]
/// Estructura de solicitud para la creación de una reserva
struct ReservationRequest {
    client_id: u32,
    room_number: u32,
    date_start: String,
    date_end: String,
    cant_integrantes: u8,
}

#[derive(Serialize)]
/// Estructura de respuesta para la disponibilidad de una fecha
struct AvailabilityResponse {
    available: bool,
}

/// Funcion que se encarga de crear un usuario
async fn create_user(info: web::Json<(String, String)>, sistema: web::Data<Arc<Sistema>>) -> impl Responder {
    let (email, password )= info.into_inner();
    if sistema.user_already_exists(&email) {
        return HttpResponse::Conflict().body(format!("User with email {} already exists", email));
    }

    sistema.add_client(email.clone(), password.clone());

    HttpResponse::Ok().body(format!("User created"))
}

/// Funcion que se encarga de verificar si un usuario existe y si la contraseña es correcta
async fn login_user(info: web::Json<(String, String)>, sistema: web::Data<Arc<Sistema>>) -> impl Responder {
    let (email, password )= info.into_inner();

    if !sistema.user_already_exists(&email) {
        return HttpResponse::Conflict().body(format!("User with email {} doesn't exists", email));
    }
    
    let user_object = sistema.user_correct_login(&email, &password);
    
    if user_object.is_none() {
        return HttpResponse::Conflict().body(format!("Incorrect password or user doesn't exists"));
    }
    else {
        // Unwrap user_object to get the Usuario and return it as JSON
        HttpResponse::Ok().json(user_object.unwrap())
    }
}

/// Funcion que se encarga de listar todas las habitaciones
async fn list_all_rooms(sistema: web::Data<Arc<Sistema>>) -> impl Responder {
    let rooms = sistema.get_all_rooms();
    print!("ListAllRooms => {:?}", rooms);
    HttpResponse::Ok().json(rooms)
}

/// Funcion que se encarga de obtener las reservas de un cliente
async fn get_reservations(info: web::Json<u32>, sistema: web::Data<Arc<Sistema>>) -> impl Responder {
    let client_id = info.into_inner();
    let reservations = sistema.get_reservation_by_client_id(client_id);
    HttpResponse::Ok().json(reservations)
}

/// Funcion que se encarga de verificar la disponibilidad de una fecha
async fn check_availability(info: web::Json<ReservationRequest>, sistema: web::Data<Arc<Sistema>>) -> impl Responder {
    let available = sistema.is_room_available(info.room_number, &info.date_start, &info.date_end);
    HttpResponse::Ok().json(AvailabilityResponse { available })
}

/// Funcion que se encarga de crear una reserva
async fn create_reservation(info: web::Json<ReservationRequest>, sistema: web::Data<Arc<Sistema>>) -> impl Responder {
    if sistema.is_room_available(info.room_number, &info.date_start, &info.date_end) {
        let reservation_id = sistema.add_reservation(
            info.client_id,
            info.room_number,
            info.date_start.clone(),
            info.date_end.clone(),
            info.cant_integrantes,
        );
        HttpResponse::Ok().json(reservation_id)
    } else {
        HttpResponse::Conflict().body("Room is not available")
    }
}

/// Funcion principal que se encarga de iniciar el servidor
#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let sistema = Arc::new(Sistema::new());

    if let Err(err) = sistema.load_vital_data() {
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
            .route("/create_user", web::post().to(create_user)) // Nueva ruta para crear usuario
            .route("/login", web::post().to(login_user))
            .route("/list_all_rooms", web::post().to(list_all_rooms))
            .route("/get_reservations", web::post().to(get_reservations))
            .route("/check", web::post().to(check_availability)) // Nueva ruta para verificar disponibilidad
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

/// Función para detener el servidor
async fn stop_server(data: web::Data<watch::Sender<bool>>) -> HttpResponse {
    println!("Stopping server...");
    let _ = data.send(true); 
    //HttpResponse::Ok().body("Server is stopping...");
    HttpResponse::Ok().finish()
    //std::process::exit(0);
}