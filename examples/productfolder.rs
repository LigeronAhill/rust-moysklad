use anyhow::Result;
use rust_moysklad::{FilterOperator, MoySkladApiClient, ProductFolder, TaxSystem};
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
    let folders = client.get_all::<ProductFolder>().await?;
    if let Some(last) = folders.last() {
        let last_folder = client.get::<ProductFolder>(last.id).await?;
        dbg!(last_folder);
    }
    if let Some(ad) = folders.iter().find(|f| f.name == "Сопутствующие товары") {
        let folder_to_create = ProductFolder::create("Ковродержатели")
            .code("42")
            .description("Очень крутое описание")
            .external_code("69")
            .product_folder(ad.meta.clone())
            .shared(true)
            .tax_system(TaxSystem::SimplifiedTaxSystemIncomeOutcome)
            .use_parent_vat(true)
            .vat(0)
            .vat_enabled(false)
            .build();
        let created: ProductFolder = client.create(folder_to_create).await?;
        dbg!(&created);
        let update = ProductFolder::update().external_code("96").build();
        let updated: ProductFolder = client.update(created.id, update).await?;
        dbg!(&updated);
        let batch = vec![ProductFolder::update()
            .meta(created.meta)
            .tax_system(TaxSystem::TaxSystemSameAsGroup)
            .build()];
        let batch_updated: Vec<ProductFolder> = client.batch_create_update(batch).await?;
        dbg!(&batch_updated);
        client.delete::<ProductFolder>(updated.id).await?;
        let search_result = client.search::<ProductFolder>("сопут").await?;
        dbg!(&search_result);
        let filter_result = client
            .filter::<ProductFolder>("pathName", FilterOperator::PartialMatch, "Ковр")
            .await?;
        dbg!(filter_result.len());
    }
    Ok(())
}
