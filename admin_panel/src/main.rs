use axum::{
    routing::{get, post},
    Router, extract::State,
    response::Html, Form
};
use sqlx::SqlitePool;
use serde::Deserialize;

#[derive(Deserialize)]
struct NewItem {
    name: String,
    price: f64,
    description: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db().await?;
    let app = Router::new()
        .route("/", get(admin_dashboard))
        .route("/add-item", post(add_item))
        .route("/orders", get(view_orders))
        .with_state(db);

    axum::Server::bind(&"127.0.0.1:8081".parse()?)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn admin_dashboard(State(db): State<SqlitePool>) -> Html<String> {
    Html(format!(r#"
        <h1>Admin Panel</h1>
        <form action="/add-item" method="post">
            <input type="text" name="name" placeholder="Name">
            <input type="number" step="0.01" name="price" placeholder="Price">
            <textarea name="description"></textarea>
            <button type="submit">Add Item</button>
        </form>
        <a href="/orders">View Orders</a>
    "#))
}

async fn add_item(State(db): State<SqlitePool>, Form(item): Form<NewItem>) {
    sqlx::query!(
        "INSERT INTO items (id, name, price, description) VALUES (?, ?, ?, ?)",
        uuid::Uuid::new_v4().to_string(),
        item.name,
        item.price,
        item.description
    ).execute(&db).await.unwrap();
}