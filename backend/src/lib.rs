pub mod models;

use axum::{
    Router,
    routing::get,
    extract::State  // Добавляем импорт State
};
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct AppState {
    pub message: Arc<RwLock<String>>,
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(root_handler))
        .with_state(state)
}

async fn root_handler(
    State(state): State<AppState>  // Теперь State будет распознан
) -> String {
    state.message.read().unwrap().clone()
}