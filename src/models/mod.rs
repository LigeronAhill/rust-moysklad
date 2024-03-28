use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::api_client::MsEntity;

pub mod assortment;
pub mod characteristic;
pub mod counterparty;
pub mod country;
pub mod currency;
pub mod product;
pub mod product_folder;
pub mod region;
pub mod uom;
pub mod variant;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub href: String,
    pub metadata_href: Option<String>,
    #[serde(rename = "type")]
    pub meta_type: Option<String>,
    pub media_type: String,
    pub uuid_href: Option<String>,
    pub download_href: Option<String>,
    pub size: Option<i32>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityResponse<T> {
    pub context: Option<Context>,
    pub meta: Meta,
    pub rows: Vec<T>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    pub employee: MetaWrapper,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaWrapper {
    pub meta: Meta,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaxSystem {
    #[default]
    GeneralTaxSystem,
    PatentBased,
    PresumptiveTaxSystem,
    SimplifiedTaxSystemIncome,
    SimplifiedTaxSystemIncomeOutcome,
    TaxSystemSameAsGroup,
    UnifiedAgriculturalTax,
}
pub fn deserialize_date_from_str<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let date_str = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S%.3f")
        .map_err(serde::de::Error::custom)
}
pub fn deserialize_option_date_from_str<'de, D>(
    deserializer: D,
) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let date_str = Option::<String>::deserialize(deserializer)?;
    match date_str {
        Some(str) => NaiveDateTime::parse_from_str(&str, "%Y-%m-%d %H:%M:%S%.3f")
            .map(Some)
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}
/// Дополнительные поля
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductsCustomField {
    pub custom_entity_meta: Option<Meta>,
    pub meta: Meta,
    pub id: uuid::Uuid,
    pub name: String,
    #[serde(rename = "type")]
    pub attribute_type: AttributeType,
    pub required: bool,
    pub description: Option<String>,
}
impl MsEntity for ProductsCustomField {
    fn url() -> String {
        String::from("https://api.moysklad.ru/api/remap/1.2/entity/product/metadata/attributes")
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomEntity {
    pub account_id: uuid::Uuid,
    pub code: Option<String>,
    pub description: Option<String>,
    pub external_code: String,
    pub id: uuid::Uuid,
    pub meta: Meta,
    pub name: String,
    #[serde(deserialize_with = "deserialize_date_from_str")]
    pub updated: NaiveDateTime,
    pub group: MetaWrapper,
    pub owner: MetaWrapper,
    pub shared: Option<bool>,
}
// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(untagged)]
// pub enum Attribute {
//     StringAttribute(StringAttribute),
//     CustomAttribute(CustomAttribute),
//     IntAttribute(IntAttribute),
//     FloatAttribute(FloatAttribute),
//     BooleanAttribute(BooleanAttribute),
//     DateAttribute(DateAttribute),
//     FileAttribute(FileAttribute),
// }
// impl Attribute {
//     pub fn from_products_custom_field<T>(field: ProductsCustomField, value: T) -> Attribute
//     where
//         T: ToString,
//     {
//         match field.attribute_type {
//             AttributeType::String => Attribute::StringAttribute(StringAttribute {
//                 meta: field.meta,
//                 id: field.id,
//                 name: field.name,
//                 attribute_type: field.attribute_type,
//                 value: value.to_string(),
//             }),
//             AttributeType::Customentity => todo!(),
//             AttributeType::Long => todo!(),
//             AttributeType::Time => todo!(),
//             AttributeType::File => todo!(),
//             AttributeType::Double => todo!(),
//             AttributeType::Boolean => todo!(),
//             AttributeType::Text => todo!(),
//             AttributeType::Link => todo!(),
//         }
//     }
// }

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct Attribute {
    pub meta: Meta,
    pub id: uuid::Uuid,
    pub name: String,
    #[serde(rename = "type")]
    pub attribute_type: AttributeType,
    pub value: AttributeValue,
    pub download: Option<DownloadMeta>,
}
impl Attribute {
    pub fn from_field(field: &ProductsCustomField, value: AttributeValue) -> Self {
        Self {
            meta: field.meta.clone(),
            id: field.id,
            name: field.name.clone(),
            attribute_type: field.attribute_type.clone(),
            value,
            download: None,
        }
    }
}
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct CustomAttribute {
//     pub meta: Meta,
//     pub id: uuid::Uuid,
//     pub name: String,
//     #[serde(rename = "type")]
//     pub attribute_type: AttributeType,
//     pub value: AttributeValue,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct StringAttribute {
//     pub meta: Meta,
//     pub id: uuid::Uuid,
//     pub name: String,
//     #[serde(rename = "type")]
//     pub attribute_type: AttributeType,
//     pub value: String,
// }
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct IntAttribute {
//     pub meta: Meta,
//     pub id: uuid::Uuid,
//     pub name: String,
//     #[serde(rename = "type")]
//     pub attribute_type: AttributeType,
//     pub value: i64,
// }
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct FloatAttribute {
//     pub meta: Meta,
//     pub id: uuid::Uuid,
//     pub name: String,
//     #[serde(rename = "type")]
//     pub attribute_type: AttributeType,
//     pub value: f64,
// }
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct BooleanAttribute {
//     pub meta: Meta,
//     pub id: uuid::Uuid,
//     pub name: String,
//     #[serde(rename = "type")]
//     pub attribute_type: AttributeType,
//     pub value: bool,
// }
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct DateAttribute {
//     pub meta: Meta,
//     pub id: uuid::Uuid,
//     pub name: String,
//     #[serde(rename = "type")]
//     pub attribute_type: AttributeType,
//     #[serde(deserialize_with = "deserialize_date_from_str")]
//     pub value: NaiveDateTime,
// }
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct FileAttribute {
//     pub meta: Meta,
//     pub id: uuid::Uuid,
//     pub name: String,
//     #[serde(rename = "type")]
//     pub attribute_type: AttributeType,
//     pub value: String,
//     pub download: DownloadMeta,
// }
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadMeta {
    pub href: String,
    pub media_type: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomValue {
    pub meta: Meta,
    pub name: String,
}
impl From<CustomEntity> for CustomValue {
    fn from(value: CustomEntity) -> Self {
        Self {
            meta: value.meta,
            name: value.name,
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttributeValue {
    Custom(CustomValue),
    String(String),
    #[serde(deserialize_with = "deserialize_date_from_str")]
    Date(NaiveDateTime),
    Bool(bool),
    Float(f64),
    Int(i32),
    #[default]
    Other,
}
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct AttributeValue {
//     pub meta: Meta,
//     pub name: String,
// }
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AttributeType {
    #[default]
    String,
    Customentity,
    Long,
    Time,
    File,
    Double,
    Boolean,
    Text,
    Link,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceType {
    pub meta: Meta,
    pub id: String,
    pub name: String,
    pub external_code: String,
}
