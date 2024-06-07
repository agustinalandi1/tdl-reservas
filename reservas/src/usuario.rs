use serde::{Deserialize, Serialize};

/// Representa un usuario con un id, nombre y email.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usuario {
    pub id: u32,
    pub email: String,
    pub password: String,
}

impl Usuario {
    /// Crea un nuevo usuario.
    pub fn new(id: u32, email: String, password: String) -> Usuario {
        Usuario { id, email, password }
    }
    
    /// Obtiene el id del usuario.
    pub fn get_id(&self) -> u32 {
        self.id
    }

    /// Obtiene el email del usuario.
    pub fn get_email(&self) -> String {
        self.email.clone()
    }

    /// Obtiene el password del usuario.
    pub fn get_password(&self) -> String {
        self.password.clone()
    }

}
