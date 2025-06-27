mod tor;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting anonymous marketplace backend...");

    tor::start_hidden_service().await?;

    println!("Service is ready");
    Ok(())
}