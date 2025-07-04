mod tor;
mod api;
mod database;
use axum::http::Method;
use tower_http::cors::{CorsLayer, Any};
use axum::{Router, routing::get};
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // 1. Запуск Tor скрытого сервиса
    let (_tor_client, onion_address) = tor::setup_hidden_service().await?;
    info!("Main: Onion service running at {}", onion_address);

    // 2. Инициализация базы данных
    let db = database::init_db().await?;

    // 3. Настройка веб-сервера
    let app = Router::new()
        .merge(api::create_router())
        .with_state(db);
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let app = Router::new()
        .merge(api::create_router())
        .layer(cors)
        .with_state(db);
    info!("Starting web server on 127.0.0.1:8080");
    axum::Server::bind(&"127.0.0.1:8080".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}