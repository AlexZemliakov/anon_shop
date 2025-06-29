use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(list_products))
        .route("/:id", get(get_product))
        .route("/", post(create_product))
}

async fn list_products() -> &'static str {
    "Products list"
}

async fn get_product() -> &'static str {
    "Single product"
}

async fn create_product() -> &'static str {
    "Product created"
}