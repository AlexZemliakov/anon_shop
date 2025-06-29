pub mod admin;
pub mod products;

use axum::{Router, routing::get};

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(|| async { "Anonymous Shop API" }))
        .nest("/admin", admin::routes())
        .nest("/products", products::routes())
}