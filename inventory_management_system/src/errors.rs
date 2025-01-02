// errors.rs

use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum StoreError {
    InvalidInput(String),
    ProductNotFound(u32),
    InsufficientStock(u32),
    AuthenticationError,
    DatabaseError(String),
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StoreError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            StoreError::ProductNotFound(id) => write!(f, "Product not found with ID: {}", id),
            StoreError::InsufficientStock(id) => write!(f, "Insufficient stock for product ID: {}", id),
            StoreError::AuthenticationError => write!(f, "Authentication failed"),
            StoreError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl Error for StoreError {}