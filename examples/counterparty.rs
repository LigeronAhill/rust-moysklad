use anyhow::Result;
use rust_moysklad::{Counterparty, MoySkladApiClient};
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
    let counterparies = client.get_all::<Counterparty>().await?;
    dbg!(&counterparies);
    let search_string = "Александр";
    let search_result = client.search::<Counterparty>(search_string).await?;
    dbg!(&search_result);
    let filtered = client
        .filter::<Counterparty>(
            "name",
            rust_moysklad::FilterOperator::PartialMatch,
            search_string,
        )
        .await?;
    dbg!(&filtered);
    let cp_to_create = Counterparty::create("Иванов Иван")
        .description("Тестовый контрагент")
        .external_code("69")
        .build();
    let created: Counterparty = client.create(cp_to_create).await?;
    dbg!(&created);
    let update = Counterparty::update()
        .company_type(rust_moysklad::CompanyType::Individual)
        .build();
    let updated: Counterparty = client.update(created.id, update).await?;
    dbg!(&updated);
    client.delete::<Counterparty>(updated.id).await?;
    Ok(())
}
