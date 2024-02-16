use anyhow::Result;
use rust_moysklad::{Characteristic, Currency, MoySkladApiClient, Product, Variant};
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
    let variants = client.get_all::<Variant>().await?;
    dbg!(variants.len());
    let search_string = "carolus";
    let search_result = client.search::<Variant>(search_string).await?;
    dbg!(&search_result);
    let filtered = client
        .filter::<Variant>(
            "name",
            rust_moysklad::FilterOperator::PartialMatch,
            search_string,
        )
        .await?;
    dbg!(&filtered);
    let chars = client.get_variants_characteristics().await?;
    dbg!(&chars);
    let price_types = client.get_price_types().await?;
    let products = client.search::<Product>("Краска для разметки").await?;
    let currencies = client.get_all::<Currency>().await?;
    if let Some(char) = chars.iter().find(|c| c.name == "Ширина рулона, м") {
        if let Some(product) = products.first() {
            let characteristic = Characteristic::from_variant_char(char.clone(), 4);
            let mut variant_to_create = Variant::create(product.meta.clone(), vec![characteristic]);
            if let Some(sale_price) = price_types.iter().find(|p| p.name == "Цена продажи")
            {
                if let Some(rub) = currencies.iter().find(|c| c.iso_code == "RUB") {
                    variant_to_create.sale_price(500000.0, &rub.meta, &sale_price.meta);
                }
            }
            let vtc = variant_to_create.build();
            let created: Variant = client.create(vtc).await?;
            dbg!(&created);
            let update = Variant::update().description("Test description").build();
            let updated: Variant = client.update(created.id, update).await?;
            dbg!(&updated);
            client.delete::<Variant>(updated.id).await?;
        }
    }
    Ok(())
}
