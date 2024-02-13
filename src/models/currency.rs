use serde::{Deserialize, Serialize};

use crate::api_client::MsEntity;

use super::Meta;
/// Средствами JSON API можно запрашивать списки валют и сведения по отдельным валютам, а также создавать новые и обновлять сведения по уже существующим валютам. Кодом сущности для валют в составе JSON API является ключевое слово currency. По данной сущности можно осуществлять контекстный поиск с помощью специального параметра search. Поиск с параметром search отличается от других тем, что поиск не префиксный, без токенизации и идет только по одному полю одновременно. Ищет такие строки, в которые входит значение строки поиска.
/// # Example
///
/// ```rust
/// use anyhow::Result;
/// use rust_moysklad::{Currency, MoySkladApiClient};
/// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let client = MoySkladApiClient::from_env().expect("MS_TOKEN env var not set!");
///     tracing_subscriber::registry()
///         .with(
///             tracing_subscriber::EnvFilter::try_from_default_env()
///                 .unwrap_or_else(|_| "rust-moysklad=debug".into()),
///         )
///         .with(tracing_subscriber::fmt::layer())
///         .init();
///     let currencies = client.get_all::<Currency>().await?;
///     if let Some(last) = currencies.last() {
///         dbg!(last);
///     }
///     Ok(())
/// }
/// ```
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    /// Добавлена ли Валюта в архив
    pub archived: bool,
    /// Цифровой код Валюты
    pub code: String,
    /// Является ли валюта валютой учета
    #[serde(rename = "default")]
    pub is_default: bool,
    /// Полное наименование Валюты
    pub full_name: String,
    /// ID Валюты
    pub id: uuid::Uuid,
    /// Признак обратного курса Валюты
    pub indirect: bool,
    /// Буквенный код Валюты
    pub iso_code: String,
    /// Формы единиц целой части Валюты.
    pub major_unit: MajorUnit,
    /// Наценка при автоматическом обновлении курса
    pub margin: Option<f64>,
    /// Метаданные Валюты
    pub meta: Meta,
    /// Формы единиц дробной части Валюты.
    pub minor_unit: MinorUnit,
    /// Кратность курса Валюты
    pub multiplicity: i32,
    /// Краткое наименование Валюты
    pub name: String,
    /// Курс Валюты
    pub rate: f64,
    /// Способ обновления курса Валюты. auto или manual
    pub rate_update_type: String,
    /// Основана ли валюта на валюте из системного справочника
    pub system: bool,
}
impl MsEntity for Currency {
    fn url() -> String {
        String::from("https://api.moysklad.ru/api/remap/1.2/entity/currency")
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MajorUnit {
    pub gender: Option<String>,
    pub s1: Option<String>,
    pub s2: Option<String>,
    pub s5: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinorUnit {
    pub gender: String,
    pub s1: Option<String>,
    pub s2: Option<String>,
    pub s5: Option<String>,
}
