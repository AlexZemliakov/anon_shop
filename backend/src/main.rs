use axum::{Router, routing::get};
use std::net::SocketAddr;
use arti_client::TorClient;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Запуск Tor-клиента
    let tor_client = TorClient::create_bootstrapped().await.unwrap();

    // Настройка Axum
    let app = Router::new().route("/", get(|| async { "Hello, Tor!" }));

    // Слушаем только localhost, Tor будет проксировать наружу
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}