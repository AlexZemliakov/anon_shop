// order.rs
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OrderStatus {
    New,
    Completed,
}


#[derive(Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub product_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub status: OrderStatus,
    pub comment: Option<String>,
}