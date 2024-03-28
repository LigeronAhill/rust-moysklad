use anyhow::Result;
use rust_moysklad::{MoySkladApiClient, Product};
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
    let products = client.get_all::<Product>().await?;
    dbg!(products.len());
    // let search_string = "carolus";
    // let search_result = client.search::<Product>(search_string).await?;
    // dbg!(&search_result);
    // let filtered = client
    //     .filter::<Product>(
    //         "name",
    //         rust_moysklad::FilterOperator::PartialMatch,
    //         search_string,
    //     )
    //     .await?;
    // dbg!(&filtered);
    // let price_types = client.get_price_types().await?;
    // let product_custom_fields = client.get_all::<ProductsCustomField>().await?;
    // let currencies = client.get_all::<Currency>().await?;
    // let uoms = client.get_all::<Uom>().await?;
    // let countries = client.get_all::<Country>().await?;
    // let product_folders = client.get_all::<ProductFolder>().await?;
    // let mut p_to_create = Product::create("Test Product");
    // p_to_create
    //     .external_code("testcode")
    //     .article("testarticle")
    //     .weight(2.8);
    // if let Some(brand_field) = product_custom_fields.iter().find(|f| f.name == "Бренд") {
    //     if let Some(customentitymeta) = brand_field.custom_entity_meta.to_owned() {
    //         let brands = client.get_custom_entities(&customentitymeta).await?;
    //         if let Some(aw) = brands.iter().find(|v| v.name == "AW") {
    //             let brand = Attribute::from_field(
    //                 brand_field,
    //                 AttributeValue::Custom(CustomValue::from(aw.clone())),
    //             );
    //             p_to_create.attribute(brand);
    //         }
    //     }
    // }
    // if let Some(sale_price) = price_types.iter().find(|p| p.name == "Цена продажи") {
    //     if let Some(rub) = currencies.iter().find(|c| c.iso_code == "RUB") {
    //         p_to_create.sale_price(50000.0, &rub.meta, &sale_price.meta);
    //     }
    // }
    // if let Some(m2) = uoms.iter().find(|u| u.name == "м2") {
    //     p_to_create.uom(&m2.meta);
    // }
    // if let Some(belgium) = countries.iter().find(|r| r.name == "Бельгия") {
    //     p_to_create.country(&belgium.meta);
    // }
    // if let Some(carpet_folder) = product_folders
    //     .iter()
    //     .find(|f| f.name == "Ковровые покрытия")
    // {
    //     p_to_create.product_folder(&carpet_folder.meta);
    // }
    // let product_to_create = p_to_create.build();
    // let created: Product = client.create(product_to_create).await?;
    // dbg!(&created);
    // let update = Product::update().description("Test description").build();
    // let updated: Product = client.update(created.id, update).await?;
    // dbg!(&updated);
    // client.delete::<Product>(updated.id).await?;
    Ok(())
}
