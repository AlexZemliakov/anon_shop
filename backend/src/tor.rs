use arti_client::{TorClient, TorClientConfig};
use std::{error::Error, path::PathBuf};
use tokio::fs;
use log::{info, error};
use tor_hsservice::{HsIdKeypair, OnionService};

const KEY_FILE: &str = "onion_service_private.key";

pub async fn setup_hidden_service() -> Result<(TorClient, String), Box<dyn Error>> {
    // 1. Загрузка или генерация ключа
    let key = match fs::read(KEY_FILE).await {
        Ok(k) => {
            info!("Using existing onion service key");
            k
        }
        Err(_) => {
            info!("Generating new onion service key");
            let new_key = generate_key().await?;
            fs::write(KEY_FILE, &new_key).await?;
            new_key
        }
    };

    // 2. Настройка Tor клиента
    let config = TorClientConfig::builder()
        .storage()
        .permanent()
        .data_dir(PathBuf::from("./tor-data"))?
        .build()?;

    let tor_client = TorClient::create_bootstrapped(config).await?;

    // 3. Создание скрытого сервиса
    let hidden_service = tor_client
        .create_onion_service()
        .key(key)
        .ports(vec![("80", "127.0.0.1:8080".parse()?)])  // Проброс на локальный сервер
        .await?;

    let onion_address = hidden_service.onion_address().to_string();
    info!("Onion service created: {}", onion_address);

    Ok((tor_client, onion_address))
}

async fn generate_key() -> Result<Vec<u8>, Box<dyn Error>> {
    let keypair = HsIdKeypair::generate();
    Ok(keypair.to_bytes().to_vec())
}