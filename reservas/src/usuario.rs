use serde::{Deserialize, Serialize};

/// Representa un usuario con un id, nombre y email.
#[derive(Serialize, Deserialize)]
pub struct Usuario {
    pub id: u32,
    pub name: String,
    pub email: String,
}

impl Usuario {
    /// Crea un nuevo usuario.
    pub fn new(id: u32, name: String, email: String) -> Usuario {
        Usuario { id, name, email }
    }
}
