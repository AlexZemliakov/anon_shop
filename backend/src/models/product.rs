use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub id: Uuid,
    pub name: String,
    pub price: f64,
    pub description: String,
    pub category: String,
    pub stock: u32,
    pub image_url: String,
    pub is_hidden: bool,
    pub encrypted_details: String,
}

impl Product {
    /// Создает новый товар с автоматически генерируемым UUID
    pub fn new(
        name: String,
        price: f64,
        description: String,
        category: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            price,
            description,
            category,
            stock: 0,
            image_url: String::new(),
            is_hidden: false,
            encrypted_details: String::new(),
        }
    }
}