use backend::{create_router, AppState};
use std::sync::{Arc, RwLock};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let state = AppState {
        message: Arc::new(RwLock::new("Hello World".to_string())),
    };

    let app = create_router(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app
    ).await.unwrap();
}