// main.rs
//use client_server::client;
/*
mod usuario;
mod reserva;
mod sistema;

use client::main as start_client;
use server::main as start_server;

#[tokio::main]
async fn main() {
    // Iniciar el servidor en un hilo separado
    let server_handle = tokio::spawn(async {
        if let Err(e) = start_server() {
            eprintln!("Error starting server: {}", e);
        }
    });

    // Iniciar el cliente en el hilo principal
    if let Err(e) = start_client() {
        eprintln!("Error running client: {}", e);
    }

    // Esperar a que el servidor termine
    if let Err(e) = server_handle.await {
        eprintln!("Error waiting for server: {}", e);
    }
}
*/
