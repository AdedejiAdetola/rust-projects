// models.rs

#[derive(Debug, Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub quantity: u32,
}

#[derive(Debug)]
pub struct Transaction {
    pub product_id: u32,
    pub quantity: u32,
    pub price: f64,
    pub date: chrono::NaiveDateTime,
    pub transaction_type: TransactionType,
}

#[derive(Debug)]
pub enum TransactionType {
    Sale,
    Purchase,
}