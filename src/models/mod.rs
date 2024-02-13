use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

pub mod assortment;
pub mod counterparty;
pub mod currency;
pub mod product_folder;
pub mod uom;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub href: String,
    pub metadata_href: Option<String>,
    #[serde(rename = "type")]
    pub meta_type: String,
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
    pub context: Context,
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
            .map(|dt| Some(dt))
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Characteristic {
    pub meta: Meta,
    pub id: String,
    pub name: String,
    pub value: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Attribute {
    StringAttribute(StringAttribute),
    CustomAttribute(CustomAttribute),
    IntAttribute(IntAttribute),
    FloatAttribute(FloatAttribute),
    BooleanAttribute(BooleanAttribute),
    DateAttribute(DateAttribute),
    FileAttribute(FileAttribute),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomAttribute {
    pub meta: Meta,
    pub id: uuid::Uuid,
    pub name: String,
    #[serde(rename = "type")]
    pub attribute_type: AttributeType,
    pub value: AttributeValue,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StringAttribute {
    pub meta: Meta,
    pub id: uuid::Uuid,
    pub name: String,
    #[serde(rename = "type")]
    pub attribute_type: AttributeType,
    pub value: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntAttribute {
    pub meta: Meta,
    pub id: uuid::Uuid,
    pub name: String,
    #[serde(rename = "type")]
    pub attribute_type: AttributeType,
    pub value: i64,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FloatAttribute {
    pub meta: Meta,
    pub id: uuid::Uuid,
    pub name: String,
    #[serde(rename = "type")]
    pub attribute_type: AttributeType,
    pub value: f64,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BooleanAttribute {
    pub meta: Meta,
    pub id: uuid::Uuid,
    pub name: String,
    #[serde(rename = "type")]
    pub attribute_type: AttributeType,
    pub value: bool,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateAttribute {
    pub meta: Meta,
    pub id: uuid::Uuid,
    pub name: String,
    #[serde(rename = "type")]
    pub attribute_type: AttributeType,
    #[serde(deserialize_with = "deserialize_date_from_str")]
    pub value: NaiveDateTime,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileAttribute {
    pub meta: Meta,
    pub id: uuid::Uuid,
    pub name: String,
    #[serde(rename = "type")]
    pub attribute_type: AttributeType,
    pub value: String,
    pub download: DownloadMeta,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadMeta {
    pub href: String,
    pub media_type: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttributeValue {
    pub meta: Meta,
    pub name: String,
}
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
