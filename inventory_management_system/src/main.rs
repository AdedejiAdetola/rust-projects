// main.rs

mod models;
mod inventory;
mod security;
mod errors;

use crate::inventory::Inventory;
use crate::security::authenticate;
use std::io;

fn main() {
    let mut inventory = Inventory::new();

    println!("Welcome to Rusty Store Inventory Management System");

    // Authentication
    println!("Please log in:");
    let mut username = String::new();
    let mut password = String::new();
    
    print!("Username: ");
    io::Write::flush(&mut io::stdout()).unwrap();
    io::stdin().read_line(&mut username).unwrap();
    
    print!("Password: ");
    io::Write::flush(&mut io::stdout()).unwrap();
    io::stdin().read_line(&mut password).unwrap();

    if !authenticate(username.trim(), password.trim()) {
        println!("Authentication failed. Exiting...");
        return;
    }

    loop {
        println!("Menu:");
        println!("1. Manage Inventory");
        println!("2. Record Sale");
        println!("3. Record Purchase");
        println!("4. Generate Reports");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => if let Err(e) = inventory.manage_inventory() {
                println!("Error: {}", e);
            },
            "2" => if let Err(e) = inventory.record_sale() {
                println!("Error: {}", e);
            },
            "3" => if let Err(e) = inventory.record_purchase() {
                println!("Error: {}", e);
            },
            "4" => inventory.generate_reports(),
            "5" => {
                println!("Exiting... Goodbye!");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}






