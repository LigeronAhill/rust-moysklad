use crate::{
    api_client::MsEntity,
    models::{Meta, MetaWrapper},
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::{
    characteristic::Characteristic, deserialize_option_date_from_str, Attribute, PriceType,
};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Assortment {
    pub meta: Meta,
    pub id: uuid::Uuid,
    pub account_id: uuid::Uuid,
    pub owner: Option<MetaWrapper>,
    pub shared: Option<bool>,
    pub group: Option<MetaWrapper>,
    #[serde(deserialize_with = "deserialize_option_date_from_str")]
    pub updated: Option<NaiveDateTime>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub external_code: Option<String>,
    pub archived: Option<bool>,
    pub path_name: Option<String>,
    pub product_folder: Option<MetaWrapper>,
    pub use_parent_vat: Option<bool>,
    pub vat: Option<i64>,
    pub vat_enabled: Option<bool>,
    pub effective_vat: Option<i64>,
    pub effective_vat_enabled: Option<bool>,
    pub uom: Option<MetaWrapper>,
    pub images: Option<MetaWrapper>,
    pub min_price: Option<MinPrice>,
    pub sale_prices: Vec<SalePrice>,
    pub supplier: Option<MetaWrapper>,
    pub attributes: Option<Vec<Attribute>>,
    pub payment_item_type: Option<String>,
    pub discount_prohibited: Option<bool>,
    pub country: Option<MetaWrapper>,
    pub buy_price: Option<BuyPrice>,
    pub article: Option<String>,
    pub weight: Option<f64>,
    pub volume: Option<f64>,
    pub barcodes: Option<Vec<Barcode>>,
    pub variants_count: Option<i64>,
    pub is_serial_trackable: Option<bool>,
    pub tracking_type: Option<String>,
    pub files: Option<MetaWrapper>,
    pub stock: Option<f64>,
    pub reserve: Option<f64>,
    pub in_transit: Option<f64>,
    pub quantity: Option<f64>,
    pub label: Option<String>,
    pub assortment: Option<MetaWrapper>,
    pub components: Option<MetaWrapper>,
    pub characteristics: Option<Vec<Characteristic>>,
    pub product: Option<MetaWrapper>,
}
impl MsEntity for Assortment {
    fn url() -> String {
        String::from("https://api.moysklad.ru/api/remap/1.2/entity/assortment")
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinPrice {
    pub value: f64,
    pub currency: MetaWrapper,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SalePrice {
    pub value: f64,
    pub currency: MetaWrapper,
    pub price_type: PriceType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuyPrice {
    pub value: f64,
    pub currency: MetaWrapper,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Barcode {
    pub ean13: String,
}
