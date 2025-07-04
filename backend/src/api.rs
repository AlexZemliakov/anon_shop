use axum::{
    Router, routing::{get, post},
    extract::{State, Json},
    response::Html,
    http::StatusCode
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

// Структуры данных
#[derive(Serialize, Deserialize)]
pub struct Item {
    id: String,
    name: String,
    price: f64,
    description: String,
}

#[derive(Deserialize)]
pub struct OrderRequest {
    item_id: String,
    contact_info: String, // Зашифрованные данные
}

// Маршрутизация
pub fn create_router() -> Router<SqlitePool> {
    Router::new()
        .route("/", get(index_handler))
        .route("/items", get(list_items))
        .route("/order", post(create_order))
}

// Обработчики
async fn index_handler() -> Html<&'static str> {
    Html(r#"
        <h1>Anon Shop</h1>
        <p>API Endpoints:</p>
        <ul>
            <li>GET /items - List products</li>
            <li>POST /order - Create order</li>
        </ul>
    "#)
}

async fn list_items(State(db): State<SqlitePool>) -> Json<Vec<Item>> {
    // Заглушка - потом заменим на реальные данные из БД
    Json(vec![
        Item {
            id: "1".to_string(),
            name: "Test Product".to_string(),
            price: 0.01,
            description: "Example item".to_string(),
        }
    ])
}

async fn create_order(
    State(db): State<SqlitePool>,
    Json(payload): Json<OrderRequest>
) -> StatusCode {
    // Заглушка для логики заказа
    println!("New order for item: {}", payload.item_id);
    StatusCode::CREATED
}
// Добавляем новые маршруты
pub fn create_router() -> Router<SqlitePool> {
    Router::new()
        // ... предыдущие маршруты
        .route("/admin/orders", get(list_orders))
        .route("/admin/items", post(add_item))
}

async fn list_orders(State(db): State<SqlitePool>) -> Json<Vec<Order>> {
    // Реализация запроса к БД
}

async fn add_item(State(db): State<SqlitePool>, Json(item): Json<NewItem>) -> StatusCode {
    // Логика добавления
}