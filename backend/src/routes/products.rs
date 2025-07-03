use axum::{
    extract::{State, Path, Json},
    http::StatusCode,
    response::IntoResponse,
};
use crate::models::Product;
use crate::repositories::ProductRepository;
use std::sync::Arc;
use uuid::Uuid;

pub async fn list_products(
    State(repo): State<Arc<ProductRepository>>
) -> Json<Vec<Product>> {
    Json(repo.list_products())
}

pub async fn get_product(
    State(repo): State<Arc<ProductRepository>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match repo.get_product(id) {
        Some(product) => (StatusCode::OK, Json(product)).into_response(),
        None => (StatusCode::NOT_FOUND, "Product not found").into_response(),
    }
}

pub async fn add_product(
    State(repo): State<Arc<ProductRepository>>,
    Json(product): Json<Product>,
) -> impl IntoResponse {
    match repo.add_product(product) {
        Ok(_) => (StatusCode::CREATED, "Product created").into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}

pub async fn update_product(
    State(repo): State<Arc<ProductRepository>>,
    Path(id): Path<Uuid>,
    Json(product): Json<Product>,
) -> impl IntoResponse {
    // Ensure the product ID in the path matches the product ID in the body
    let product_with_id = Product { id, ..product };

    match repo.update_product(id, product_with_id) {
        Ok(_) => (StatusCode::OK, "Product updated").into_response(),
        Err(e) => (StatusCode::NOT_FOUND, e).into_response(),
    }
}

pub async fn delete_product(
    State(repo): State<Arc<ProductRepository>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match repo.delete_product(id) {
        Ok(_) => (StatusCode::OK, "Product deleted").into_response(),
        Err(e) => (StatusCode::NOT_FOUND, e).into_response(),
    }
}
