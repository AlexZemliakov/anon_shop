// order.rs
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OrderStatus {
    New,
    Completed,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub id: Uuid,
    pub product_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub status: OrderStatus,
    pub comment: Option<String>,
}