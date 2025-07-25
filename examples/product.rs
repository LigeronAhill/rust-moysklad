use rust_moysklad::{MoySkladApiClient, Product};
use tracing::info;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    info!("Requesting products");
    let client = MoySkladApiClient::from_env().expect("MS_TOKEN env var not set!");
    let search_string = "carolus";
    let search_result = client.list::<Product>(30, 0, Some(search_string)).await?;
    info!(
        "Got {size} results of search for '{search_string}'",
        size = search_result.meta.size.unwrap_or_default()
    );
    if let Some(first_product) = search_result.rows.first() {
        info!("First founded:");
        info!(
            "Name: '{name}'",
            name = first_product
                .name
                .as_ref()
                .unwrap_or(&String::from("NoName"))
        );
        let price = first_product
            .sale_prices
            .first()
            .map(|p| p.value)
            .unwrap_or_default()
            / 100.;
        info!("Price: {price:.2}");
        let id = first_product.id;
        let get_result = client.get::<Product>(id).await?;
        assert_eq!(get_result.id, id);
    }

    Ok(())
}
