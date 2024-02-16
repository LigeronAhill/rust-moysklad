use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::api_client::MsEntity;

use super::{
    assortment::{Barcode, BuyPrice, MinPrice, SalePrice},
    characteristic::Characteristic,
    deserialize_option_date_from_str,
    product::{CreateSalePrice, Pack},
    Meta, MetaWrapper,
};
/// Модификация
///
/// # Example
///
/// ```rust
/// use anyhow::Result;
/// use rust_moysklad::{Characteristic, Currency, MoySkladApiClient, Product, Variant};
/// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     tracing_subscriber::registry()
///         .with(
///             tracing_subscriber::EnvFilter::try_from_default_env()
///                 .unwrap_or_else(|_| "rust-moysklad=debug".into()),
///         )
///         .with(tracing_subscriber::fmt::layer())
///         .init();
///     let client = MoySkladApiClient::from_env().expect("MS_TOKEN env var not set!");
///     let variants = client.get_all::<Variant>().await?;
///     dbg!(variants.len());
///     let search_string = "carolus";
///     let search_result = client.search::<Variant>(search_string).await?;
///     dbg!(&search_result);
///     let filtered = client
///         .filter::<Variant>(
///             "name",
///             rust_moysklad::FilterOperator::PartialMatch,
///             search_string,
///         )
///         .await?;
///     dbg!(&filtered);
///     let chars = client.get_variants_characteristics().await?;
///     dbg!(&chars);
///     let price_types = client.get_price_types().await?;
///     let products = client.search::<Product>("Краска для разметки").await?;
///     let currencies = client.get_all::<Currency>().await?;
///     if let Some(char) = chars.iter().find(|c| c.name == "Ширина рулона, м") {
///         if let Some(product) = products.first() {
///             let characteristic = Characteristic::from_variant_char(char.clone(), 4);
///             let mut variant_to_create = Variant::create(product.meta.clone(), vec![characteristic]);
///             if let Some(sale_price) = price_types.iter().find(|p| p.name == "Цена продажи")
///             {
///                 if let Some(rub) = currencies.iter().find(|c| c.iso_code == "RUB") {
///                     variant_to_create.sale_price(500000.0, &rub.meta, &sale_price.meta);
///                 }
///             }
///             let vtc = variant_to_create.build();
///             let created: Variant = client.create(vtc).await?;
///             dbg!(&created);
///             let update = Variant::update().description("Test description").build();
///             let updated: Variant = client.update(created.id, update).await?;
///             dbg!(&updated);
///             client.delete::<Variant>(updated.id).await?;
///         }
///     }
///     Ok(())
/// }
/// ```
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Variant {
    pub account_id: uuid::Uuid,
    pub archived: bool,
    pub barcodes: Option<Vec<Barcode>>,
    pub buy_price: Option<BuyPrice>,
    pub characteristics: Vec<Characteristic>,
    pub code: Option<String>,
    pub description: Option<String>,
    pub discount_prohibited: bool,
    pub external_code: String,
    pub id: uuid::Uuid,
    pub images: Option<MetaWrapper>,
    pub meta: Meta,
    pub min_price: Option<MinPrice>,
    pub name: String,
    pub packs: Option<Vec<Pack>>,
    pub product: MetaWrapper,
    pub sale_prices: Vec<SalePrice>,
    pub things: Option<Vec<String>>,
    #[serde(deserialize_with = "deserialize_option_date_from_str")]
    pub updated: Option<NaiveDateTime>,
}
impl Variant {
    pub fn create(product: Meta, characteristics: Vec<Characteristic>) -> CreateVariantBuilder {
        CreateVariantBuilder {
            product: MetaWrapper { meta: product },
            characteristics,
            ..Default::default()
        }
    }
    pub fn update() -> UpdateVariantBuilder {
        UpdateVariantBuilder::default()
    }
}
impl MsEntity for Variant {
    fn url() -> String {
        String::from("https://api.moysklad.ru/api/remap/1.2/entity/variant")
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateVariant {
    #[serde(skip_serializing_if = "Option::is_none")]
    barcodes: Option<Vec<Barcode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    buy_price: Option<BuyPrice>,
    characteristics: Vec<Characteristic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discount_prohibited: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    images: Option<MetaWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_price: Option<MinPrice>,
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    packs: Option<Vec<Pack>>,
    product: MetaWrapper,
    sale_prices: Option<Vec<CreateSalePrice>>,
}
#[derive(Default)]
pub struct CreateVariantBuilder {
    barcodes: Option<Vec<Barcode>>,
    buy_price: Option<BuyPrice>,
    characteristics: Vec<Characteristic>,
    code: Option<String>,
    description: Option<String>,
    discount_prohibited: Option<bool>,
    external_code: Option<String>,
    images: Option<MetaWrapper>,
    min_price: Option<MinPrice>,
    name: Option<String>,
    packs: Option<Vec<Pack>>,
    product: MetaWrapper,
    sale_prices: Option<Vec<CreateSalePrice>>,
}
impl CreateVariantBuilder {
    pub fn barcode<T>(&mut self, barcode: T) -> &mut Self
    where
        T: ToString,
    {
        self.barcodes.get_or_insert(vec![]).push(Barcode {
            ean13: barcode.to_string(),
        });
        self
    }
    pub fn buy_price(&mut self, value: f64, currency: Meta) -> &mut Self {
        let _ = self.buy_price.insert(BuyPrice {
            value,
            currency: MetaWrapper { meta: currency },
        });
        self
    }
    pub fn characteristic(&mut self, char: Characteristic) -> &mut Self {
        self.characteristics.push(char);
        self
    }
    pub fn code<T>(&mut self, code: T) -> &mut Self
    where
        T: ToString,
    {
        let _ = self.code.insert(code.to_string());
        self
    }
    pub fn description<T>(&mut self, description: T) -> &mut Self
    where
        T: ToString,
    {
        let _ = self.description.insert(description.to_string());
        self
    }
    pub fn discount_prohibited(&mut self, discount_prohibited: bool) -> &mut Self {
        let _ = self.discount_prohibited.insert(discount_prohibited);
        self
    }
    pub fn external_code<T>(&mut self, external_code: T) -> &mut Self
    where
        T: ToString,
    {
        let _ = self.external_code.insert(external_code.to_string());
        self
    }
    pub fn min_price(&mut self, value: f64, currency: Meta) -> &mut Self {
        let _ = self.min_price.insert(MinPrice {
            value,
            currency: MetaWrapper { meta: currency },
        });
        self
    }
    pub fn name<T>(&mut self, name: T) -> &mut Self
    where
        T: ToString,
    {
        let _ = self.name.insert(name.to_string());
        self
    }
    pub fn sale_price(
        &mut self,
        value: f64,
        currency_meta: &Meta,
        price_type_meta: &Meta,
    ) -> &mut Self {
        self.sale_prices
            .get_or_insert(vec![])
            .push(CreateSalePrice {
                value,
                currency: MetaWrapper {
                    meta: currency_meta.to_owned(),
                },
                price_type: MetaWrapper {
                    meta: price_type_meta.to_owned(),
                },
            });
        self
    }
    pub fn build(&self) -> CreateVariant {
        CreateVariant {
            barcodes: self.barcodes.to_owned(),
            buy_price: self.buy_price.to_owned(),
            characteristics: self.characteristics.to_owned(),
            code: self.code.to_owned(),
            description: self.description.to_owned(),
            discount_prohibited: self.discount_prohibited,
            external_code: self.external_code.to_owned(),
            images: self.images.to_owned(),
            min_price: self.min_price.to_owned(),
            name: self.name.to_owned(),
            packs: self.packs.to_owned(),
            product: self.product.to_owned(),
            sale_prices: self.sale_prices.to_owned(),
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateVariant {
    #[serde(skip_serializing_if = "Option::is_none")]
    barcodes: Option<Vec<Barcode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    buy_price: Option<BuyPrice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    characteristics: Option<Vec<Characteristic>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discount_prohibited: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<uuid::Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    images: Option<MetaWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<Meta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_price: Option<MinPrice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    packs: Option<Vec<Pack>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product: Option<MetaWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sale_prices: Option<Vec<CreateSalePrice>>,
}
#[derive(Default)]
pub struct UpdateVariantBuilder {
    barcodes: Option<Vec<Barcode>>,
    buy_price: Option<BuyPrice>,
    characteristics: Option<Vec<Characteristic>>,
    code: Option<String>,
    description: Option<String>,
    discount_prohibited: Option<bool>,
    external_code: Option<String>,
    id: Option<uuid::Uuid>,
    images: Option<MetaWrapper>,
    meta: Option<Meta>,
    min_price: Option<MinPrice>,
    name: Option<String>,
    packs: Option<Vec<Pack>>,
    product: Option<MetaWrapper>,
    sale_prices: Option<Vec<CreateSalePrice>>,
}
impl UpdateVariantBuilder {
    pub fn barcode<T>(&mut self, barcode: T) -> &mut Self
    where
        T: ToString,
    {
        self.barcodes.get_or_insert(vec![]).push(Barcode {
            ean13: barcode.to_string(),
        });
        self
    }
    pub fn buy_price(&mut self, value: f64, currency: Meta) -> &mut Self {
        let _ = self.buy_price.insert(BuyPrice {
            value,
            currency: MetaWrapper { meta: currency },
        });
        self
    }
    pub fn characteristic(&mut self, char: Characteristic) -> &mut Self {
        self.characteristics.get_or_insert(vec![]).push(char);
        self
    }
    pub fn code<T>(&mut self, code: T) -> &mut Self
    where
        T: ToString,
    {
        let _ = self.code.insert(code.to_string());
        self
    }
    pub fn description<T>(&mut self, description: T) -> &mut Self
    where
        T: ToString,
    {
        let _ = self.description.insert(description.to_string());
        self
    }
    pub fn discount_prohibited(&mut self, discount_prohibited: bool) -> &mut Self {
        let _ = self.discount_prohibited.insert(discount_prohibited);
        self
    }
    pub fn external_code<T>(&mut self, external_code: T) -> &mut Self
    where
        T: ToString,
    {
        let _ = self.external_code.insert(external_code.to_string());
        self
    }
    pub fn id(&mut self, id: uuid::Uuid) -> &mut Self {
        let _ = self.id.insert(id);
        self
    }
    pub fn meta(&mut self, meta: Meta) -> &mut Self {
        let _ = self.meta.insert(meta);
        self
    }
    pub fn min_price(&mut self, value: f64, currency: Meta) -> &mut Self {
        let _ = self.min_price.insert(MinPrice {
            value,
            currency: MetaWrapper { meta: currency },
        });
        self
    }
    pub fn name<T>(&mut self, name: T) -> &mut Self
    where
        T: ToString,
    {
        let _ = self.name.insert(name.to_string());
        self
    }
    pub fn product(&mut self, product_meta: Meta) -> &mut Self {
        let _ = self.product.insert(MetaWrapper { meta: product_meta });
        self
    }
    pub fn sale_price(
        &mut self,
        value: f64,
        currency_meta: &Meta,
        price_type_meta: &Meta,
    ) -> &mut Self {
        self.sale_prices
            .get_or_insert(vec![])
            .push(CreateSalePrice {
                value,
                currency: MetaWrapper {
                    meta: currency_meta.to_owned(),
                },
                price_type: MetaWrapper {
                    meta: price_type_meta.to_owned(),
                },
            });
        self
    }
    pub fn build(&self) -> UpdateVariant {
        UpdateVariant {
            barcodes: self.barcodes.to_owned(),
            buy_price: self.buy_price.to_owned(),
            characteristics: self.characteristics.to_owned(),
            code: self.code.to_owned(),
            description: self.description.to_owned(),
            discount_prohibited: self.discount_prohibited,
            external_code: self.external_code.to_owned(),
            images: self.images.to_owned(),
            min_price: self.min_price.to_owned(),
            name: self.name.to_owned(),
            packs: self.packs.to_owned(),
            product: self.product.to_owned(),
            sale_prices: self.sale_prices.to_owned(),
            id: self.id,
            meta: self.meta.to_owned(),
        }
    }
}
