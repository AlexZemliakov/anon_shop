use anyhow::Result;
use arti_client::{TorClient, BootstrapBehavior};
use arti_client::config::TorClientConfig;
use tokio::runtime::Runtime;

pub async fn create_tor_client() -> Result<TorClient<impl arti_client::Runtime>> {
    // 1. Создаем конфигурацию
    let config = TorClientConfig::default();

    // 2. Получаем runtime (асинхронное окружение)
    let runtime = Runtime::new()?;

    
    // 3. Создаем Tor клиент
    let tor_client = TorClient::with_runtime(runtime)
        .config(config)
        .bootstrap_behavior(BootstrapBehavior::OnDemand)
        .create()?;

    Ok(tor_client)
}

pub async fn start_hidden_service() -> Result<()> {
    let _tor_client = create_tor_client().await?;
    println!("✅ Tor client initialized successfully");
    Ok(())
}