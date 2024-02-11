use serde::{Deserialize, Serialize};

pub mod assortment;

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
