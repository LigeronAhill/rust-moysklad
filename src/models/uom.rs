use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::api_client::MsEntity;

use super::{deserialize_date_from_str, Meta, MetaWrapper};
/// Единица измерения
///
/// # Example
///
/// ```rust
/// use anyhow::Result;
/// use rust_moysklad::{MoySkladApiClient, Uom};
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
///     let uoms = client.get_all::<Uom>().await?;
///     dbg!(&uoms);
///     let search_string = "Квадратный метр";
///     let m2 = client.search::<Uom>(search_string).await?;
///     dbg!(&m2);
///     let filtered = client
///         .filter::<Uom>(
///             "description",
///             rust_moysklad::FilterOperator::Equal,
///             search_string,
///         )
///         .await?;
///     dbg!(&filtered);
///     Ok(())
/// }
/// ```
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Uom {
    /// ID учетной записи    
    pub account_id: Option<uuid::Uuid>,
    /// Код Единицы измерения
    pub code: String,
    /// Описание Единциы измерения
    pub description: String,
    /// Внешний код Единицы измерения
    pub external_code: String,
    /// Отдел сотрудника
    pub group: Option<MetaWrapper>,
    /// ID Единицы измерения
    pub id: uuid::Uuid,
    /// Метаданные Единицы измерения
    pub meta: Meta,
    /// Наименование Единицы измерения
    pub name: String,
    /// Владелец (Сотрудник)
    pub owner: Option<MetaWrapper>,
    /// Общий доступ
    pub shared: Option<bool>,
    /// Момент последнего обновления Единицы измерения
    #[serde(deserialize_with = "deserialize_date_from_str")]
    pub updated: NaiveDateTime,
}
impl MsEntity for Uom {
    fn url() -> String {
        String::from("https://api.moysklad.ru/api/remap/1.2/entity/uom")
    }
}
