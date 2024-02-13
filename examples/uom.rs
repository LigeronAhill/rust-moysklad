use anyhow::Result;
use rust_moysklad::{MoySkladApiClient, Uom};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust-moysklad=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let client = MoySkladApiClient::from_env().expect("MS_TOKEN env var not set!");
    let uoms = client.get_all::<Uom>().await?;
    dbg!(&uoms);
    let search_string = "Квадратный метр";
    let m2 = client.search::<Uom>(search_string).await?;
    dbg!(&m2);
    let filtered = client
        .filter::<Uom>(
            "description",
            rust_moysklad::FilterOperator::Equal,
            search_string,
        )
        .await?;
    dbg!(&filtered);
    Ok(())
}
