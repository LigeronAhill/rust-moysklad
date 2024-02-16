use std::fmt::{Debug, Display};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    models::{CustomEntity, EntityResponse, Meta},
    PriceType,
};
/// initialize api client
///
/// # Example
///
/// ```rust
/// use anyhow::Result;
/// use rust_moysklad::{Assortment, MoySkladApiClient};
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
///     let assortment = client.get_all::<Assortment>().await?;
///     if let Some(last) = assortment.last() {
///         dbg!(last);
///     }
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct MoySkladApiClient {
    token: String,
}
pub trait MsEntity: for<'a> Deserialize<'a> + Serialize + Clone + Debug {
    fn url() -> String;
}
impl MoySkladApiClient {
    /// initialize api client
    ///
    /// # Example
    ///
    /// ```rust
    /// use anyhow::Result;
    /// use rust_moysklad::MoySkladApiClient;
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client = MoySkladApiClient::from_env()?;
    ///     //...do something...
    ///     Ok(())
    /// }
    /// ```
    pub fn from_env() -> Result<Self> {
        let token = std::env::var("MS_TOKEN")?;
        Ok(Self { token })
    }
    /// retrieve list of entity
    ///
    /// # Example
    ///
    /// ```rust
    /// use anyhow::Result;
    /// use rust_moysklad::{Assortment, MoySkladApiClient};
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
    ///     let assortment = client.get_all::<Assortment>().await?;
    ///     // ...do something...
    ///     Ok(())
    /// }
    /// ```
    #[instrument]
    pub async fn get_all<E>(&self) -> Result<Vec<E>>
    where
        E: MsEntity,
    {
        static APP_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .gzip(true)
            .build()?;
        let limit = 1000;
        let mut offset = 0;
        let mut result = Vec::new();
        loop {
            let response = client
                .get(E::url())
                .bearer_auth(&self.token)
                .query(&[(limit, offset)])
                .send()
                .await?;
            match response.status() {
                reqwest::StatusCode::OK => {
                    let res: EntityResponse<E> = response.json().await?;
                    if res.rows.is_empty() {
                        break;
                    } else {
                        result.extend(res.rows);
                        if let Some(size) = res.meta.size {
                            if limit + offset > size {
                                break;
                            }
                        }
                        offset += limit;
                    }
                }
                _ => {
                    let err_res: serde_json::Value = response.json().await?;
                    let msg = format!("{err_res:#?}\n");
                    return Err(anyhow::Error::msg(msg));
                }
            }
        }
        Ok(result)
    }
    /// get entity
    ///
    /// # Example
    ///
    /// ```rust
    /// use anyhow::Result;
    /// use rust_moysklad::{Currency, MoySkladApiClient};
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
    ///     let currencies = client.get_all::<Currency>().await?;
    ///     if let Some(last) = currencies.last() {
    ///         let last_currency = client.get::<Currency>(last.id).await?;
    ///         dbg!(last_currency);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    #[instrument]
    pub async fn get<E>(&self, id: Uuid) -> Result<E>
    where
        E: MsEntity,
    {
        static APP_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .gzip(true)
            .build()?;
        let uri = format!("{}/{id}", E::url());
        let response = client.get(&uri).bearer_auth(&self.token).send().await?;
        match response.status() {
            reqwest::StatusCode::OK => {
                let res: E = response.json().await?;
                Ok(res)
            }
            _ => {
                let err_res: serde_json::Value = response.json().await?;
                let msg = format!("{err_res:#?}\n");
                Err(anyhow::Error::msg(msg))
            }
        }
    }
    /// Create entity
    ///
    /// # Example
    ///
    /// ```rust
    /// use anyhow::Result;
    /// use rust_moysklad::{FilterOperator, MoySkladApiClient, ProductFolder, TaxSystem};
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
    ///     let folders = client.get_all::<ProductFolder>().await?;
    ///     if let Some(last) = folders.last() {
    ///         let last_folder = client.get::<ProductFolder>(last.id).await?;
    ///         dbg!(last_folder);
    ///     }
    ///     if let Some(ad) = folders.iter().find(|f| f.name == "Сопутствующие товары") {
    ///         let folder_to_create = ProductFolder::create("Ковродержатели")
    ///             .code("42")
    ///             .description("Очень крутое описание")
    ///             .external_code("69")
    ///             .product_folder(ad.meta.clone())
    ///             .shared(true)
    ///             .tax_system(TaxSystem::SimplifiedTaxSystemIncomeOutcome)
    ///             .use_parent_vat(true)
    ///             .vat(0)
    ///             .vat_enabled(false)
    ///             .build();
    ///         let created: ProductFolder = client.create(folder_to_create).await?;
    ///         dbg!(&created);
    ///         let update = ProductFolder::update().external_code("96").build();
    ///         let updated: ProductFolder = client.update(created.id, update).await?;
    ///         dbg!(&updated);
    ///         let batch = vec![ProductFolder::update()
    ///             .meta(created.meta)
    ///             .tax_system(TaxSystem::TaxSystemSameAsGroup)
    ///             .build()];
    ///         let batch_updated: Vec<ProductFolder> = client.batch_create_update(batch).await?;
    ///         dbg!(&batch_updated);
    ///         client.delete::<ProductFolder>(updated.id).await?;
    ///         let search_result = client.search::<ProductFolder>("сопут").await?;
    ///         dbg!(&search_result);
    ///         let filter_result = client
    ///             .filter::<ProductFolder>("pathName", FilterOperator::PartialMatch, "Ковр")
    ///             .await?;
    ///         dbg!(filter_result.len());
    ///     }
    ///     Ok(())
    /// }
    /// ```
    #[instrument]
    pub async fn create<E, C>(&self, object: C) -> Result<E>
    where
        E: MsEntity,
        C: Serialize + Debug,
    {
        static APP_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .gzip(true)
            .build()?;
        let response = client
            .post(E::url())
            .json(&object)
            .bearer_auth(&self.token)
            .send()
            .await?;
        match response.status() {
            reqwest::StatusCode::OK => {
                let res: E = response.json().await?;
                Ok(res)
            }
            _ => {
                let err_res: serde_json::Value = response.json().await?;
                let msg = format!("{err_res:#?}\n");
                Err(anyhow::Error::msg(msg))
            }
        }
    }
    /// Update entity
    ///
    /// # Example
    ///
    /// ```rust
    /// use anyhow::Result;
    /// use rust_moysklad::{FilterOperator, MoySkladApiClient, ProductFolder, TaxSystem};
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
    ///     let folders = client.get_all::<ProductFolder>().await?;
    ///     if let Some(last) = folders.last() {
    ///         let last_folder = client.get::<ProductFolder>(last.id).await?;
    ///         dbg!(last_folder);
    ///     }
    ///     if let Some(ad) = folders.iter().find(|f| f.name == "Сопутствующие товары") {
    ///         let folder_to_create = ProductFolder::create("Ковродержатели")
    ///             .code("42")
    ///             .description("Очень крутое описание")
    ///             .external_code("69")
    ///             .product_folder(ad.meta.clone())
    ///             .shared(true)
    ///             .tax_system(TaxSystem::SimplifiedTaxSystemIncomeOutcome)
    ///             .use_parent_vat(true)
    ///             .vat(0)
    ///             .vat_enabled(false)
    ///             .build();
    ///         let created: ProductFolder = client.create(folder_to_create).await?;
    ///         dbg!(&created);
    ///         let update = ProductFolder::update().external_code("96").build();
    ///         let updated: ProductFolder = client.update(created.id, update).await?;
    ///         dbg!(&updated);
    ///         let batch = vec![ProductFolder::update()
    ///             .meta(created.meta)
    ///             .tax_system(TaxSystem::TaxSystemSameAsGroup)
    ///             .build()];
    ///         let batch_updated: Vec<ProductFolder> = client.batch_create_update(batch).await?;
    ///         dbg!(&batch_updated);
    ///         client.delete::<ProductFolder>(updated.id).await?;
    ///         let search_result = client.search::<ProductFolder>("сопут").await?;
    ///         dbg!(&search_result);
    ///         let filter_result = client
    ///             .filter::<ProductFolder>("pathName", FilterOperator::PartialMatch, "Ковр")
    ///             .await?;
    ///         dbg!(filter_result.len());
    ///     }
    ///     Ok(())
    /// }
    /// ```
    #[instrument]
    pub async fn update<E, U>(&self, id: Uuid, object: U) -> Result<E>
    where
        E: MsEntity,
        U: Debug + Serialize,
    {
        static APP_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .gzip(true)
            .build()?;
        let uri = format!("{}/{id}", E::url());
        let response = client
            .put(&uri)
            .bearer_auth(&self.token)
            .json(&object)
            .send()
            .await?;
        match response.status() {
            reqwest::StatusCode::OK => {
                let res: E = response.json().await?;
                Ok(res)
            }
            _ => {
                let err_res: serde_json::Value = response.json().await?;
                let msg = format!("{err_res:#?}\n");
                Err(anyhow::Error::msg(msg))
            }
        }
    }
    /// Delete entity
    ///
    /// # Example
    ///
    /// ```rust
    /// use anyhow::Result;
    /// use rust_moysklad::{FilterOperator, MoySkladApiClient, ProductFolder, TaxSystem};
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
    ///     let folders = client.get_all::<ProductFolder>().await?;
    ///     if let Some(last) = folders.last() {
    ///         let last_folder = client.get::<ProductFolder>(last.id).await?;
    ///         dbg!(last_folder);
    ///     }
    ///     if let Some(ad) = folders.iter().find(|f| f.name == "Сопутствующие товары") {
    ///         let folder_to_create = ProductFolder::create("Ковродержатели")
    ///             .code("42")
    ///             .description("Очень крутое описание")
    ///             .external_code("69")
    ///             .product_folder(ad.meta.clone())
    ///             .shared(true)
    ///             .tax_system(TaxSystem::SimplifiedTaxSystemIncomeOutcome)
    ///             .use_parent_vat(true)
    ///             .vat(0)
    ///             .vat_enabled(false)
    ///             .build();
    ///         let created: ProductFolder = client.create(folder_to_create).await?;
    ///         dbg!(&created);
    ///         let update = ProductFolder::update().external_code("96").build();
    ///         let updated: ProductFolder = client.update(created.id, update).await?;
    ///         dbg!(&updated);
    ///         let batch = vec![ProductFolder::update()
    ///             .meta(created.meta)
    ///             .tax_system(TaxSystem::TaxSystemSameAsGroup)
    ///             .build()];
    ///         let batch_updated: Vec<ProductFolder> = client.batch_create_update(batch).await?;
    ///         dbg!(&batch_updated);
    ///         client.delete::<ProductFolder>(updated.id).await?;
    ///         let search_result = client.search::<ProductFolder>("сопут").await?;
    ///         dbg!(&search_result);
    ///         let filter_result = client
    ///             .filter::<ProductFolder>("pathName", FilterOperator::PartialMatch, "Ковр")
    ///             .await?;
    ///         dbg!(filter_result.len());
    ///     }
    ///     Ok(())
    /// }
    /// ```
    #[instrument]
    pub async fn delete<E>(&self, id: Uuid) -> Result<()>
    where
        E: MsEntity,
    {
        static APP_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .gzip(true)
            .build()?;
        let uri = format!("{}/{id}", E::url());
        let response = client.delete(&uri).bearer_auth(&self.token).send().await?;
        match response.status() {
            reqwest::StatusCode::OK => Ok(()),
            _ => {
                let err_res: serde_json::Value = response.json().await?;
                let msg = format!("{err_res:#?}\n");
                Err(anyhow::Error::msg(msg))
            }
        }
    }
    /// Batch create/update entities
    /// for updates required meta field
    ///
    /// # Example
    ///
    /// ```rust
    /// use anyhow::Result;
    /// use rust_moysklad::{FilterOperator, MoySkladApiClient, ProductFolder, TaxSystem};
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
    ///     let folders = client.get_all::<ProductFolder>().await?;
    ///     if let Some(last) = folders.last() {
    ///         let last_folder = client.get::<ProductFolder>(last.id).await?;
    ///         dbg!(last_folder);
    ///     }
    ///     if let Some(ad) = folders.iter().find(|f| f.name == "Сопутствующие товары") {
    ///         let folder_to_create = ProductFolder::create("Ковродержатели")
    ///             .code("42")
    ///             .description("Очень крутое описание")
    ///             .external_code("69")
    ///             .product_folder(ad.meta.clone())
    ///             .shared(true)
    ///             .tax_system(TaxSystem::SimplifiedTaxSystemIncomeOutcome)
    ///             .use_parent_vat(true)
    ///             .vat(0)
    ///             .vat_enabled(false)
    ///             .build();
    ///         let created: ProductFolder = client.create(folder_to_create).await?;
    ///         dbg!(&created);
    ///         let update = ProductFolder::update().external_code("96").build();
    ///         let updated: ProductFolder = client.update(created.id, update).await?;
    ///         dbg!(&updated);
    ///         let batch = vec![ProductFolder::update()
    ///             .meta(created.meta)
    ///             .tax_system(TaxSystem::TaxSystemSameAsGroup)
    ///             .build()];
    ///         let batch_updated: Vec<ProductFolder> = client.batch_create_update(batch).await?;
    ///         dbg!(&batch_updated);
    ///         client.delete::<ProductFolder>(updated.id).await?;
    ///         let search_result = client.search::<ProductFolder>("сопут").await?;
    ///         dbg!(&search_result);
    ///         let filter_result = client
    ///             .filter::<ProductFolder>("pathName", FilterOperator::PartialMatch, "Ковр")
    ///             .await?;
    ///         dbg!(filter_result.len());
    ///     }
    ///     Ok(())
    /// }
    /// ```
    #[instrument]
    pub async fn batch_create_update<E, C>(&self, objects: Vec<C>) -> Result<Vec<E>>
    where
        E: MsEntity,
        C: Serialize + Debug,
    {
        static APP_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .gzip(true)
            .build()?;
        let response = client
            .post(E::url())
            .json(&objects)
            .bearer_auth(&self.token)
            .send()
            .await?;
        match response.status() {
            reqwest::StatusCode::OK => {
                let res: Vec<E> = response.json().await?;
                Ok(res)
            }
            _ => {
                let err_res: serde_json::Value = response.json().await?;
                let msg = format!("{err_res:#?}\n");
                Err(anyhow::Error::msg(msg))
            }
        }
    }
    pub async fn batch_delete<E>(&self, objects: Vec<impl Serialize>) -> Result<()>
    where
        E: MsEntity,
    {
        static APP_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .gzip(true)
            .build()?;
        let uri = format!("{}/delete", E::url());
        let response = client
            .post(&uri)
            .json(&objects)
            .bearer_auth(&self.token)
            .send()
            .await?;
        match response.status() {
            reqwest::StatusCode::OK => Ok(()),
            _ => {
                let err_res: serde_json::Value = response.json().await?;
                let msg = format!("{err_res:#?}\n");
                Err(anyhow::Error::msg(msg))
            }
        }
    }
    /// Контекстный поиск
    /// В JSON API можно осуществлять контекстный поиск среди списка сущностей определенного типа по их строковым полям. Для этого используется URI параметр фильтрации search
    ///
    /// search Параметр фильтрации, с помощью которого можно осуществить поиск в списке сущностей. Поиск происходит по основным строковым полям сущностей данного типа. Результатом поиска будет отсортированный по релевантности список сущностей данного типа, прошедших фильтрацию по переданной поисковой строке. В отличии от фильтрации выборки с помощью параметра filter, при которой значения проверяются на точное совпадение указанным, при контекстном поиске проверка на совпадение не строгая. Таким образом, если осуществлять фильтрацию вида ../entity/?filter=name=120 в отфильтрованную выборку попадут только те сущности, поле name у которых имеет значение 120 и никакие другие. При контекстном поиске вида ../entity/?search=120 будут выведены как сущности с name равным 120, так и сущности, в имени (или в другом строковом поле) которых 120 является началом какого-то слова, например 12003, пазл детский 1200 штук и т.п. Причем, если ввести несколько слов ../entity/?search=120 возврат и поиск идет по полям name и description, то будут выведены как сущности с name равным 1200 и с description равным возврат из-за деффекта, так и сущности с именем 777 с описанием розничный возврат на улице 120 летия.
    ///
    /// Примеры запросов контекстного поиска (значения должны быть urlencoded):
    /// https://api.moysklad.ru/api/remap/1.2/entity/project?search=реструктуризация
    /// https://api.moysklad.ru/api/remap/1.2/entity/move?search=ул.Вавилова
    /// https://api.moysklad.ru/api/remap/1.2/entity/counterparty?search=петров
    /// use anyhow::Result;
    /// use rust_moysklad::{FilterOperator, MoySkladApiClient, ProductFolder, TaxSystem};
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
    ///     let folders = client.get_all::<ProductFolder>().await?;
    ///     if let Some(last) = folders.last() {
    ///         let last_folder = client.get::<ProductFolder>(last.id).await?;
    ///         dbg!(last_folder);
    ///     }
    ///     if let Some(ad) = folders.iter().find(|f| f.name == "Сопутствующие товары") {
    ///         let folder_to_create = ProductFolder::create("Ковродержатели")
    ///             .code("42")
    ///             .description("Очень крутое описание")
    ///             .external_code("69")
    ///             .product_folder(ad.meta.clone())
    ///             .shared(true)
    ///             .tax_system(TaxSystem::SimplifiedTaxSystemIncomeOutcome)
    ///             .use_parent_vat(true)
    ///             .vat(0)
    ///             .vat_enabled(false)
    ///             .build();
    ///         let created: ProductFolder = client.create(folder_to_create).await?;
    ///         dbg!(&created);
    ///         let update = ProductFolder::update().external_code("96").build();
    ///         let updated: ProductFolder = client.update(created.id, update).await?;
    ///         dbg!(&updated);
    ///         let batch = vec![ProductFolder::update()
    ///             .meta(created.meta)
    ///             .tax_system(TaxSystem::TaxSystemSameAsGroup)
    ///             .build()];
    ///         let batch_updated: Vec<ProductFolder> = client.batch_create_update(batch).await?;
    ///         dbg!(&batch_updated);
    ///         client.delete::<ProductFolder>(updated.id).await?;
    ///         let search_result = client.search::<ProductFolder>("сопут").await?;
    ///         dbg!(&search_result);
    ///         let filter_result = client
    ///             .filter::<ProductFolder>("pathName", FilterOperator::PartialMatch, "Ковр")
    ///             .await?;
    ///         dbg!(filter_result.len());
    ///     }
    ///     Ok(())
    /// }
    pub async fn search<E>(&self, search_string: impl Into<String>) -> Result<Vec<E>>
    where
        E: MsEntity,
    {
        static APP_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .gzip(true)
            .build()?;
        let limit = 1000;
        let mut offset = 0;
        let mut result = Vec::new();
        let search_string: String = search_string.into();
        loop {
            let response = client
                .get(E::url())
                .bearer_auth(&self.token)
                .query(&[
                    (limit.to_string(), offset.to_string()),
                    ("search".to_string(), search_string.clone()),
                ])
                .send()
                .await?;
            match response.status() {
                reqwest::StatusCode::OK => {
                    let res: EntityResponse<E> = response.json().await?;
                    if res.rows.is_empty() {
                        break;
                    } else {
                        result.extend(res.rows);
                        if let Some(size) = res.meta.size {
                            if limit + offset > size {
                                break;
                            }
                        }
                        offset += limit;
                    }
                }
                _ => {
                    let err_res: serde_json::Value = response.json().await?;
                    let msg = format!("{err_res:#?}\n");
                    return Err(anyhow::Error::msg(msg));
                }
            }
        }
        Ok(result)
    }
    /// Фильтрация выборки с помощью параметра filter
    /// Для фильтрации выборки по нескольким полям можно использовать url параметр filter. Значение этого параметра - urlencoded строка с поисковыми условиями, перечисленными через ;. Для использования самого символа ; в текстовых фильтрах необходимо указывать два символа \;. (Все примеры ниже указаны без urlencoded для лучшей читаемости) Каждое поисковое условие - это сочетание названия поля, оператора и константы. Фильтровать можно по всем полям, значения которых являются примитивными типами. Т.е. нельзя фильтровать поля-объекты и поля-массивы, все остальные поля могут быть использованы в параметре filter.
    ///
    /// Допустимые операторы: ['=', '>', '<', '>=', '<=', '!=', '~', '~=', '=~']
    /// Если в поисковом запросе несколько раз встречается условие типа "равенство" = примененное к одному и тому же полю, то такое условие интерпретируется как совокупность условий, разделенных логическим оператором ИЛИ.
    ///
    /// Например условие filter=sum=100;sum=150 будет интерпретировано как sum=100 ИЛИ sum=150 или же sum in (100, 150)
    /// Если же встречается несколько условий вида "не равно" !=, наложенных на одну и ту же переменную, то они интерпретируются как совокупность условий разделенных логическим оператором И.
    ///
    /// Например условие filter=name!=0001;name!=0002 будет эквивалентно следующим (взаимно эквивалентным) условиям : name != 0001 И name != 0002 или name not in (0001, 0002)
    /// Если на одно из полей наложено ограничение типа "равенство", а затем на него накладывается ограничение типа неравенство - в таком случае произойдет ошибка.
    ///
    /// Например условие filter=sum=100;sum>99 вызовет ошибку.
    /// Допускается использование одновременно нескольких одинаковых операторов сравнения ['>', '<', '>=', '<='] для одного поля. При этом будет использовано лишь первое значение.
    ///
    /// Например условие filter=sum>99;sum>100 будет аналогично условию filter=sum>99. В будущих версиях такое условие будет вызывать ошибку.
    /// Фильтры, примененные к разным полям объединяются через логическое И, т.е. в запросе вида:
    ///
    /// filter=sum=100;moment>2016-10-11 12:00:00 выборка будет отфильтрована и по сумме и по дате.
    ///
    /// # Example
    ///
    /// ```rust
    /// use anyhow::Result;
    /// use rust_moysklad::{FilterOperator, MoySkladApiClient, ProductFolder, TaxSystem};
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
    ///     let folders = client.get_all::<ProductFolder>().await?;
    ///     if let Some(last) = folders.last() {
    ///         let last_folder = client.get::<ProductFolder>(last.id).await?;
    ///         dbg!(last_folder);
    ///     }
    ///     if let Some(ad) = folders.iter().find(|f| f.name == "Сопутствующие товары") {
    ///         let folder_to_create = ProductFolder::create("Ковродержатели")
    ///             .code("42")
    ///             .description("Очень крутое описание")
    ///             .external_code("69")
    ///             .product_folder(ad.meta.clone())
    ///             .shared(true)
    ///             .tax_system(TaxSystem::SimplifiedTaxSystemIncomeOutcome)
    ///             .use_parent_vat(true)
    ///             .vat(0)
    ///             .vat_enabled(false)
    ///             .build();
    ///         let created: ProductFolder = client.create(folder_to_create).await?;
    ///         dbg!(&created);
    ///         let update = ProductFolder::update().external_code("96").build();
    ///         let updated: ProductFolder = client.update(created.id, update).await?;
    ///         dbg!(&updated);
    ///         let batch = vec![ProductFolder::update()
    ///             .meta(created.meta)
    ///             .tax_system(TaxSystem::TaxSystemSameAsGroup)
    ///             .build()];
    ///         let batch_updated: Vec<ProductFolder> = client.batch_create_update(batch).await?;
    ///         dbg!(&batch_updated);
    ///         client.delete::<ProductFolder>(updated.id).await?;
    ///         let search_result = client.search::<ProductFolder>("сопут").await?;
    ///         dbg!(&search_result);
    ///         let filter_result = client
    ///             .filter::<ProductFolder>("pathName", FilterOperator::PartialMatch, "Ковр")
    ///             .await?;
    ///         dbg!(filter_result.len());
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn filter<E>(
        &self,
        field: impl Into<String>,
        operator: FilterOperator,
        value: impl Into<String>,
    ) -> Result<Vec<E>>
    where
        E: MsEntity,
    {
        static APP_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .gzip(true)
            .build()?;
        let limit = 1000;
        let mut offset = 0;
        let mut result = Vec::new();
        let search_string = format!("{}{}{}", field.into(), operator.to_string(), value.into());
        loop {
            let response = client
                .get(E::url())
                .bearer_auth(&self.token)
                .query(&[
                    (limit.to_string(), offset.to_string()),
                    ("filter".to_string(), search_string.clone()),
                ])
                .send()
                .await?;
            match response.status() {
                reqwest::StatusCode::OK => {
                    let res: EntityResponse<E> = response.json().await?;
                    if res.rows.is_empty() {
                        break;
                    } else {
                        result.extend(res.rows);
                        if let Some(size) = res.meta.size {
                            if limit + offset > size {
                                break;
                            }
                        }
                        offset += limit;
                    }
                }
                _ => {
                    let err_res: serde_json::Value = response.json().await?;
                    let msg = format!("{err_res:#?}\n");
                    return Err(anyhow::Error::msg(msg));
                }
            }
        }
        Ok(result)
    }
    /// Типы цен
    pub async fn get_price_types(&self) -> Result<Vec<PriceType>> {
        let uri = "https://api.moysklad.ru/api/remap/1.2/context/companysettings/pricetype";
        static APP_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .gzip(true)
            .build()?;
        let result: Vec<PriceType> = client
            .get(uri)
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;
        Ok(result)
    }
    /// Получить элементы справочника
    pub async fn get_custom_entities(&self, customentity_meta: &Meta) -> Result<Vec<CustomEntity>> {
        let path = customentity_meta.href.clone();
        let id_vec = path.split('/').collect::<Vec<&str>>();
        let id = id_vec
            .last()
            .ok_or(anyhow::Error::msg("error getting dictionary id"))?;
        let uri = format!("https://api.moysklad.ru/api/remap/1.2/entity/customentity/{id}");
        static APP_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .gzip(true)
            .build()?;
        let response = client.get(uri).bearer_auth(&self.token).send().await?;
        match response.status() {
            reqwest::StatusCode::OK => {
                let res: EntityResponse<CustomEntity> = response.json().await?;
                Ok(res.rows)
            }
            _ => {
                let err_res: serde_json::Value = response.json().await?;
                let msg = format!("{err_res:#?}\n");
                Err(anyhow::Error::msg(msg))
            }
        }
    }
}
/// Доступные операторы для фильтрации
pub enum FilterOperator {
    /// `=` - фильтрация по значению
    Equal,
    /// `~` - частичное совпадение
    PartialMatch,
    /// `!~` - частичное совпадение не выводится
    NoPartialMatch,
    /// `~=` - полное совпадение в начале значения
    FullMatchAtTheBeginning,
    /// `=~` - полное совпадение в конце значения
    CompleteMatchAtTheEnd,
    /// `>` - больше
    GreaterThan,
    /// `<` - меньше
    LesserThan,
    /// `>=` - больше или равно
    GreaterThanOrEqual,
    /// `<=` - меньше или равно
    LesserThanOrEqual,
}
impl Display for FilterOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterOperator::Equal => write!(f, "="),
            FilterOperator::PartialMatch => write!(f, "~"),
            FilterOperator::NoPartialMatch => write!(f, "!~"),
            FilterOperator::FullMatchAtTheBeginning => write!(f, "~="),
            FilterOperator::CompleteMatchAtTheEnd => write!(f, "=~"),
            FilterOperator::GreaterThan => write!(f, ">"),
            FilterOperator::LesserThan => write!(f, "<"),
            FilterOperator::GreaterThanOrEqual => write!(f, ">="),
            FilterOperator::LesserThanOrEqual => write!(f, "<="),
        }
    }
}
