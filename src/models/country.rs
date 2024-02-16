use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::api_client::MsEntity;

use super::{deserialize_date_from_str, Meta, MetaWrapper};

/// Страны
/// Средствами JSON API можно создавать и обновлять сведения о Странах, запрашивать списки Стран и сведения по отдельным Странам. Кодом сущности для Страны в составе JSON API является ключевое слово country.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    pub account_id: Option<uuid::Uuid>,
    pub code: Option<String>,
    pub description: Option<String>,
    pub external_code: String,
    pub group: Option<MetaWrapper>,
    pub id: uuid::Uuid,
    pub meta: Meta,
    pub name: String,
    pub owner: Option<MetaWrapper>,
    pub shared: Option<bool>,
    #[serde(deserialize_with = "deserialize_date_from_str")]
    pub updated: NaiveDateTime,
}
impl MsEntity for Country {
    fn url() -> String {
        String::from("https://api.moysklad.ru/api/remap/1.2/entity/country")
    }
}
