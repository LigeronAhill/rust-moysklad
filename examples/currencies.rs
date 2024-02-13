use anyhow::Result;
use rust_moysklad::{Currency, MoySkladApiClient};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
#[tokio::main]
async fn main() -> Result<()> {
    let client = MoySkladApiClient::from_env().expect("MS_TOKEN env var not set!");
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust-moysklad=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let currencies = client.get_all::<Currency>().await?;
    if let Some(last) = currencies.last() {
        let last_currency = client.get::<Currency>(last.id).await?;
        dbg!(last_currency);
    }
    Ok(())
}
