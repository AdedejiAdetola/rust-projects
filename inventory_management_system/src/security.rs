// security.rs

use std::collections::HashMap;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref USERS: Mutex<HashMap<String, String>> = {
        let mut m = HashMap::new();
        // Default admin credentials - in a real system, these would be stored securely
        m.insert("admin".to_string(), "password123".to_string());
        Mutex::new(m)
    };
}

pub fn authenticate(username: &str, password: &str) -> bool {
    if let Ok(users) = USERS.lock() {
        if let Some(stored_password) = users.get(username) {
            return stored_password == password;
        }
    }
    false
}