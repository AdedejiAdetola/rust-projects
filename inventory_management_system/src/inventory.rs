// inventory.rs

use crate::models::{Product, Transaction, TransactionType};
use crate::errors::StoreError;
use chrono::Local;
use std::collections::HashMap;
use std::io;

pub struct Inventory {
    pub products: HashMap<u32, Product>,
    pub transactions: Vec<Transaction>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            products: HashMap::new(),
            transactions: Vec::new(),
        }
    }

    pub fn manage_inventory(&mut self) -> Result<(), StoreError> {
        println!("Inventory Management:");
        println!("1. Add Product");
        println!("2. Edit Product");
        println!("3. Delete Product");
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => self.add_product(),
            "2" => self.edit_product(),
            "3" => self.delete_product(),
            _ => Err(StoreError::InvalidInput("Invalid menu choice".to_string())),
        }
    }

    fn add_product(&mut self) -> Result<(), StoreError> {
        let mut id = String::new();
        let mut name = String::new();
        let mut description = String::new();
        let mut price = String::new();
        let mut quantity = String::new();

        println!("Enter Product ID:");
        io::stdin().read_line(&mut id).unwrap();
        let id: u32 = id.trim().parse()
            .map_err(|_| StoreError::InvalidInput("Invalid product ID".to_string()))?;

        if self.products.contains_key(&id) {
            return Err(StoreError::InvalidInput("Product ID already exists".to_string()));
        }

        println!("Enter Product Name:");
        io::stdin().read_line(&mut name).unwrap();
        println!("Enter Product Description:");
        io::stdin().read_line(&mut description).unwrap();
        
        println!("Enter Product Price:");
        io::stdin().read_line(&mut price).unwrap();
        let price: f64 = price.trim().parse()
            .map_err(|_| StoreError::InvalidInput("Invalid price".to_string()))?;

        println!("Enter Product Quantity:");
        io::stdin().read_line(&mut quantity).unwrap();
        let quantity: u32 = quantity.trim().parse()
            .map_err(|_| StoreError::InvalidInput("Invalid quantity".to_string()))?;

        let product = Product {
            id,
            name: name.trim().to_string(),
            description: description.trim().to_string(),
            price,
            quantity,
        };

        self.products.insert(id, product);
        println!("Product added successfully!");
        Ok(())
    }

    fn edit_product(&mut self) -> Result<(), StoreError> {
        let mut id = String::new();
        println!("Enter Product ID to edit:");
        io::stdin().read_line(&mut id).unwrap();
        let id: u32 = id.trim().parse()
            .map_err(|_| StoreError::InvalidInput("Invalid product ID".to_string()))?;

        let product = self.products.get_mut(&id)
            .ok_or(StoreError::ProductNotFound(id))?;

        let mut name = String::new();
        let mut description = String::new();
        let mut price = String::new();
        let mut quantity = String::new();

        println!("Enter new Product Name (current: {}):", product.name);
        io::stdin().read_line(&mut name).unwrap();
        if !name.trim().is_empty() {
            product.name = name.trim().to_string();
        }

        println!("Enter new Product Description (current: {}):", product.description);
        io::stdin().read_line(&mut description).unwrap();
        if !description.trim().is_empty() {
            product.description = description.trim().to_string();
        }

        println!("Enter new Product Price (current: {}):", product.price);
        io::stdin().read_line(&mut price).unwrap();
        if !price.trim().is_empty() {
            product.price = price.trim().parse()
                .map_err(|_| StoreError::InvalidInput("Invalid price".to_string()))?;
        }

        println!("Enter new Product Quantity (current: {}):", product.quantity);
        io::stdin().read_line(&mut quantity).unwrap();
        if !quantity.trim().is_empty() {
            product.quantity = quantity.trim().parse()
                .map_err(|_| StoreError::InvalidInput("Invalid quantity".to_string()))?;
        }

        println!("Product updated successfully!");
        Ok(())
    }

    fn delete_product(&mut self) -> Result<(), StoreError> {
        let mut id = String::new();
        println!("Enter Product ID to delete:");
        io::stdin().read_line(&mut id).unwrap();
        let id: u32 = id.trim().parse()
            .map_err(|_| StoreError::InvalidInput("Invalid product ID".to_string()))?;

        if self.products.remove(&id).is_some() {
            println!("Product deleted successfully!");
            Ok(())
        } else {
            Err(StoreError::ProductNotFound(id))
        }
    }

    pub fn record_sale(&mut self) -> Result<(), StoreError> {
        println!("Recording a sale:");
        let mut id = String::new();
        let mut quantity = String::new();
        let mut price = String::new();

        println!("Enter Product ID:");
        io::stdin().read_line(&mut id).unwrap();
        let id: u32 = id.trim().parse()
            .map_err(|_| StoreError::InvalidInput("Invalid product ID".to_string()))?;

        println!("Enter Quantity Sold:");
        io::stdin().read_line(&mut quantity).unwrap();
        let quantity: u32 = quantity.trim().parse()
            .map_err(|_| StoreError::InvalidInput("Invalid quantity".to_string()))?;

        println!("Enter Sale Price per Unit:");
        io::stdin().read_line(&mut price).unwrap();
        let price: f64 = price.trim().parse()
            .map_err(|_| StoreError::InvalidInput("Invalid price".to_string()))?;

        let product = self.products.get_mut(&id)
            .ok_or(StoreError::ProductNotFound(id))?;

        if product.quantity < quantity {
            return Err(StoreError::InsufficientStock(id));
        }

        product.quantity -= quantity;
        let transaction = Transaction {
            product_id: id,
            quantity,
            price,
            date: Local::now().naive_local(),
            transaction_type: TransactionType::Sale,
        };
        self.transactions.push(transaction);
        println!("Sale recorded successfully!");
        Ok(())
    }

    pub fn record_purchase(&mut self) -> Result<(), StoreError> {
        println!("Recording a purchase:");
        let mut id = String::new();
        let mut quantity = String::new();
        let mut price = String::new();

        println!("Enter Product ID:");
        io::stdin().read_line(&mut id).unwrap();
        let id: u32 = id.trim().parse()
            .map_err(|_| StoreError::InvalidInput("Invalid product ID".to_string()))?;

        println!("Enter Quantity Purchased:");
        io::stdin().read_line(&mut quantity).unwrap();
        let quantity: u32 = quantity.trim().parse()
            .map_err(|_| StoreError::InvalidInput("Invalid quantity".to_string()))?;

        println!("Enter Purchase Price per Unit:");
        io::stdin().read_line(&mut price).unwrap();
        let price: f64 = price.trim().parse()
            .map_err(|_| StoreError::InvalidInput("Invalid price".to_string()))?;

        let product = self.products.get_mut(&id)
            .ok_or(StoreError::ProductNotFound(id))?;

        product.quantity += quantity;
        let transaction = Transaction {
            product_id: id,
            quantity,
            price,
            date: Local::now().naive_local(),
            transaction_type: TransactionType::Purchase,
        };
        self.transactions.push(transaction);
        println!("Purchase recorded successfully!");
        Ok(())
    }

    pub fn generate_reports(&self) {
        println!("\nReports Menu:");
        println!("1. Inventory Report");
        println!("2. Sales Report");
        println!("3. Purchase Report");
        println!("4. Profit Report");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        
        match choice.trim() {
            "1" => self.inventory_report(),
            "2" => self.sales_report(),
            "3" => self.purchase_report(),
            "4" => self.profit_report(),
            _ => println!("Invalid choice."),
        }
    }

    fn inventory_report(&self) {
        println!("\n=== Inventory Report ===");
        println!("{:<10} {:<20} {:<30} {:<10} {:<10}", "ID", "Name", "Description", "Price", "Quantity");
        println!("{}", "-".repeat(80));
        
        for product in self.products.values() {
            println!("{:<10} {:<20} {:<30} ${:<9.2} {:<10}",
                product.id,
                product.name,
                product.description,
                product.price,
                product.quantity
            );
        }
    }

    fn sales_report(&self) {
        println!("\n=== Sales Report ===");
        println!("{:<10} {:<10} {:<10} {:<20} {:<10}", "Product", "Quantity", "Price", "Date", "Total");
        println!("{}", "-".repeat(60));
        
        let mut total_sales = 0.0;
        for transaction in &self.transactions {
            if let TransactionType::Sale = transaction.transaction_type {
                let total = transaction.quantity as f64 * transaction.price;
                if let Some(product) = self.products.get(&transaction.product_id) {
                    println!("{:<10} {:<10} ${:<9.2} {:<20} ${:<9.2}",
                        product.name,
                        transaction.quantity,
                        transaction.price,
                        transaction.date.format("%Y-%m-%d %H:%M"),
                        total
                    );
                    total_sales += total;
                }
            }
        }
        println!("\nTotal Sales: ${:.2}", total_sales);
    }

    fn purchase_report(&self) {
        println!("\n=== Purchase Report ===");
        println!("{:<10} {:<10} {:<10} {:<20} {:<10}", "Product", "Quantity", "Price", "Date", "Total");
        println!("{}", "-".repeat(60));
        
        let mut total_purchases = 0.0;
        for transaction in &self.transactions {
            if let TransactionType::Purchase = transaction.transaction_type {
                let total = transaction.quantity as f64 * transaction.price;
                if let Some(product) = self.products.get(&transaction.product_id) {
                    println!("{:<10} {:<10} ${:<9.2} {:<20} ${:<9.2}",
                        product.name,
                        transaction.quantity,
                        transaction.price,
                        transaction.date.format("%Y-%m-%d %H:%M"),
                        total
                    );
                    total_purchases += total;
                }
            }
        }
        println!("\nTotal Purchases: ${:.2}", total_purchases);
    }

    fn profit_report(&self) {
        println!("\n=== Profit Report ===");
        let mut total_sales = 0.0;
        let mut total_purchases = 0.0;

        for transaction in &self.transactions {
            let total = transaction.quantity as f64 * transaction.price;
            match transaction.transaction_type {
                TransactionType::Sale => total_sales += total,
                TransactionType::Purchase => total_purchases += total,
            }
        }

        println!("Total Sales: ${:.2}", total_sales);
        println!("Total Purchases: ${:.2}", total_purchases);
        println!("Gross Profit: ${:.2}", total_sales - total_purchases);
    }
}
