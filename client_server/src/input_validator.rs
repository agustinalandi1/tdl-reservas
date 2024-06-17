use regex::Regex;
use chrono::NaiveDate;
use std::fmt;

pub trait Validator<T> {
    /// Valida el input y devuelve un resultado o el error.
    fn validate(&self, input: &str) -> Result<T, ValidationError>;
}


#[derive(Debug)]
/// Representa el error de validación
pub struct ValidationError {
    details: String,
}

impl ValidationError {
    /// Crea un nuevo error de validación
    fn new(msg: &str) -> ValidationError {
        ValidationError { details: msg.to_string() }
    }
}

/// Implementación de Display para ValidationError
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

// Validador para el email
pub struct EmailValidator;

impl Validator<()> for EmailValidator {
    /// Valida que el input sea un email válido
    fn validate(&self, input: &str) -> Result<(), ValidationError> {
        let email_regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
        if email_regex.is_match(input.trim()) {
            Ok(())
        } else {
            Err(ValidationError::new("Invalid email address"))
        }
    }
}

// Validador para el formato de fecha ingresada
pub struct DateValidator;

impl Validator<(i32, i32,i32)> for DateValidator {
    fn validate(&self, input: &str) -> Result<(i32, i32, i32), ValidationError> {
        if NaiveDate::parse_from_str( input.trim(), "%Y-%m-%d").is_ok() {
            let date = NaiveDate::parse_from_str(input.trim(), "%Y-%m-%d").unwrap();
            let year = date.format("%Y").to_string().parse::<i32>().unwrap();
            let month = date.format("%m").to_string().parse::<i32>().unwrap();
            let day = date.format("%d").to_string().parse::<i32>().unwrap();

            Ok((year, day, month))
        } else {
            Err(ValidationError::new("Invalid date format, expected YYYY-MM-DD"))
        }
    }
}

// Validador para la contraseña
pub struct PasswordValidator;

// Implementación de Validator para PasswordValidator
impl Validator<()> for PasswordValidator {
    fn validate(&self, input: &str) -> Result<(), ValidationError> {
        let _min_password_len = 5;
        if input.len() >= _min_password_len {
            Ok(())
        } else {
            Err(ValidationError::new("Password must be at least 5 characters long"))
        }
    }
}