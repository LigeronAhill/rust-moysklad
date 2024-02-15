use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::api_client::MsEntity;

use super::{deserialize_date_from_str, Meta};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Region {
    pub account_id: Option<uuid::Uuid>,
    pub code: Option<String>,
    pub external_code: String,
    pub id: uuid::Uuid,
    pub meta: Meta,
    pub name: String,
    #[serde(deserialize_with = "deserialize_date_from_str")]
    pub updated: NaiveDateTime,
    pub version: Option<i32>,
}
impl MsEntity for Region {
    fn url() -> String {
        String::from("https://api.moysklad.ru/api/remap/1.2/entity/region")
    }
}
