use anyhow::Result;
use rust_moysklad::{Assortment, MoySkladApiClient};
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
    let assortment = client.get_all::<Assortment>().await?;
    if let Some(last) = assortment.last() {
        dbg!(last);
    }
    dbg!(assortment.len());
    Ok(())
}
