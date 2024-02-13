use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api_client::MsEntity;

use super::{deserialize_date_from_str, Meta, MetaWrapper, TaxSystem};
/// Группы товаров
/// Средствами JSON API можно создавать и обновлять сведения о Группах товаров, запрашивать списки Групп товаров и сведения по отдельным Группам товаров. Кодом сущности для Группы товаров в составе JSON API является ключевое слово productfolder.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductFolder {
    /// ID учетной записи
    pub account_id: Uuid,
    /// Добавлена ли Группа товаров в архив
    pub archived: bool,
    /// Код Группы товаров
    pub code: Option<String>,
    /// Описание Группы товаров
    pub description: Option<String>,
    /// Реальный НДС %
    pub effective_vat: Option<i32>,
    /// Дополнительный признак для определения разграничения реального НДС = 0 или "без НДС". (effectiveVat = 0, effectiveVatEnabled = false) -> "без НДС", (effectiveVat = 0, effectiveVatEnabled = true) -> 0%.
    pub effective_vat_enabled: Option<bool>,
    /// Внешний код Группы товаров
    pub external_code: String,
    /// Метаданные отдела сотрудника
    pub group: MetaWrapper,
    /// ID Группы товаров
    pub id: Uuid,
    /// Метаданные Группы товаров
    pub meta: Meta,
    /// Наименование Группы товаров
    pub name: String,
    /// Метаданные владельца (Сотрудника)
    pub owner: MetaWrapper,
    /// Наименование Группы товаров, в которую входит данная Группа товаров
    pub path_name: String,
    /// Ссылка на Группу товаров, в которую входит данная Группа товаров, в формате Метаданных
    pub product_folder: Option<MetaWrapper>,
    /// Общий доступ
    pub shared: bool,
    /// Код системы налогообложения
    pub tax_system: Option<TaxSystem>,
    /// Момент последнего обновления сущности
    #[serde(deserialize_with = "deserialize_date_from_str")]
    pub updated: NaiveDateTime,
    /// Используется ли ставка НДС родительской группы. Если true для единицы ассортимента будет применена ставка, установленная для родительской группы.
    pub use_parent_vat: bool,
    /// НДС %
    pub vat: Option<i32>,
    /// Включен ли НДС для группы. С помощью этого флага для группы можно выставлять НДС = 0 или НДС = "без НДС". (vat = 0, vatEnabled = false) -> vat = "без НДС", (vat = 0, vatEnabled = true) -> vat = 0%.
    pub vat_enabled: Option<bool>,
}
impl MsEntity for ProductFolder {
    fn url() -> String {
        String::from("https://api.moysklad.ru/api/remap/1.2/entity/productfolder")
    }
}
impl ProductFolder {
    /// Группы товаров
    /// Средствами JSON API можно создавать и обновлять сведения о Группах товаров, запрашивать списки Групп товаров и сведения по отдельным Группам товаров. Кодом сущности для Группы товаров в составе JSON API является ключевое слово productfolder.
    ///
    /// # Example
    ///
    /// ```rust
    /// use anyhow::Result;
    /// use rust_moysklad::{MoySkladApiClient, ProductFolder, TaxSystem};
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
    ///     let folder_to_create = ProductFolder::create("Ковродержатели")
    ///         .code("42")
    ///         .description("Очень крутое описание")
    ///         .external_code("69")
    ///         .shared(true)
    ///         .tax_system(TaxSystem::SimplifiedTaxSystemIncomeOutcome)
    ///         .use_parent_vat(true)
    ///         .vat(0)
    ///         .vat_enabled(false)
    ///         .build();
    ///     let created: ProductFolder = client.create(folder_to_create).await?;
    ///     Ok(())
    /// }
    /// ```
    pub fn create(name: impl Into<String>) -> CreateProductFolderBuilder {
        CreateProductFolderBuilder::new(name)
    }
    /// Изменить Группу товаров
    /// Запрос на обновление Группы товаров с указанным id. В теле запроса можно указать только те поля, которые необходимо изменить у Группы товаров
    ///
    /// # Example
    ///
    /// ```rust
    /// use anyhow::Result;
    /// use rust_moysklad::{MoySkladApiClient, ProductFolder, TaxSystem};
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
    ///     let update = ProductFolder::update().external_code("96").build();
    ///     let updated: ProductFolder = client.update(id, update).await?;
    ///     dbg!(&updated);
    ///     Ok(())
    /// }
    /// ```
    pub fn update() -> UpdateProductFolderBuilder {
        UpdateProductFolderBuilder::default()
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProductFolder {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_code: Option<String>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_folder: Option<MetaWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shared: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_system: Option<TaxSystem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_parent_vat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vat: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vat_enabled: Option<bool>,
}
#[derive(Default)]
pub struct CreateProductFolderBuilder {
    code: Option<String>,
    description: Option<String>,
    external_code: Option<String>,
    name: String,
    product_folder: Option<MetaWrapper>,
    shared: Option<bool>,
    tax_system: Option<TaxSystem>,
    use_parent_vat: Option<bool>,
    vat: Option<i32>,
    vat_enabled: Option<bool>,
}
impl CreateProductFolderBuilder {
    /// Наименование Группы товаров
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }
    /// Код Группы товаров
    pub fn code(&mut self, code: impl Into<String>) -> &mut Self {
        let _ = self.code.insert(code.into());
        self
    }
    /// Описание Группы товаров
    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        let _ = self.description.insert(description.into());
        self
    }
    /// Внешний код Группы товаров
    pub fn external_code(&mut self, external_code: impl Into<String>) -> &mut Self {
        let _ = self.external_code.insert(external_code.into());
        self
    }
    /// Ссылка на Группу товаров, в которую входит данная Группа товаров, в формате Метаданных
    pub fn product_folder(&mut self, meta: Meta) -> &mut Self {
        let wrapper = MetaWrapper { meta };
        let _ = self.product_folder.insert(wrapper);
        self
    }
    /// Общий доступ
    pub fn shared(&mut self, shared: bool) -> &mut Self {
        let _ = self.shared.insert(shared);
        self
    }
    /// Код системы налогообложения
    pub fn tax_system(&mut self, tax_system: TaxSystem) -> &mut Self {
        let _ = self.tax_system.insert(tax_system);
        self
    }
    /// Используется ли ставка НДС родительской группы. Если true для единицы ассортимента будет применена ставка, установленная для родительской группы.
    pub fn use_parent_vat(&mut self, use_parent_vat: bool) -> &mut Self {
        let _ = self.use_parent_vat.insert(use_parent_vat);
        self
    }
    /// НДС %
    pub fn vat(&mut self, vat: i32) -> &mut Self {
        let _ = self.vat.insert(vat);
        self
    }
    /// Включен ли НДС для группы. С помощью этого флага для группы можно выставлять НДС = 0 или НДС = "без НДС". (vat = 0, vatEnabled = false) -> vat = "без НДС", (vat = 0, vatEnabled = true) -> vat = 0%.
    pub fn vat_enabled(&mut self, vat_enabled: bool) -> &mut Self {
        let _ = self.vat_enabled.insert(vat_enabled);
        self
    }
    pub fn build(&self) -> CreateProductFolder {
        CreateProductFolder {
            code: self.code.to_owned(),
            description: self.description.to_owned(),
            external_code: self.external_code.to_owned(),
            name: self.name.to_owned(),
            product_folder: self.product_folder.to_owned(),
            shared: self.shared,
            tax_system: self.tax_system.to_owned(),
            use_parent_vat: self.use_parent_vat,
            vat: self.vat,
            vat_enabled: self.vat_enabled,
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProductFolder {
    #[serde(skip_serializing_if = "Option::is_none")]
    archived: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<Meta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_folder: Option<MetaWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shared: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_system: Option<TaxSystem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_parent_vat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vat: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vat_enabled: Option<bool>,
}
#[derive(Default)]
pub struct UpdateProductFolderBuilder {
    archived: Option<bool>,
    code: Option<String>,
    description: Option<String>,
    external_code: Option<String>,
    id: Option<Uuid>,
    meta: Option<Meta>,
    name: Option<String>,
    product_folder: Option<MetaWrapper>,
    shared: Option<bool>,
    tax_system: Option<TaxSystem>,
    use_parent_vat: Option<bool>,
    vat: Option<i32>,
    vat_enabled: Option<bool>,
}
impl UpdateProductFolderBuilder {
    /// Добавлена ли Группа товаров в архив
    pub fn archived(&mut self, archived: bool) -> &mut Self {
        let _ = self.archived.insert(archived);
        self
    }
    /// Код Группы товаров
    pub fn code(&mut self, code: impl Into<String>) -> &mut Self {
        let _ = self.code.insert(code.into());
        self
    }
    /// Описание Группы товаров
    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        let _ = self.description.insert(description.into());
        self
    }
    /// Внешний код Группы товаров
    pub fn external_code(&mut self, external_code: impl Into<String>) -> &mut Self {
        let _ = self.external_code.insert(external_code.into());
        self
    }
    /// ID Группы товаров
    pub fn id(&mut self, id: Uuid) -> &mut Self {
        let _ = self.id.insert(id);
        self
    }
    /// Метаданные Группы товаров
    pub fn meta(&mut self, meta: Meta) -> &mut Self {
        let _ = self.meta.insert(meta);
        self
    }
    /// Наименование Группы товаров
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        let _ = self.name.insert(name.into());
        self
    }
    /// Ссылка на Группу товаров, в которую входит данная Группа товаров, в формате Метаданных
    pub fn product_folder(&mut self, meta: Meta) -> &mut Self {
        let w = MetaWrapper { meta };
        let _ = self.product_folder.insert(w);
        self
    }
    /// Общий доступ
    pub fn shared(&mut self, shared: bool) -> &mut Self {
        let _ = self.shared.insert(shared);
        self
    }
    /// Код системы налогообложения
    pub fn tax_system(&mut self, tax_system: TaxSystem) -> &mut Self {
        let _ = self.tax_system.insert(tax_system);
        self
    }
    /// Используется ли ставка НДС родительской группы. Если true для единицы ассортимента будет применена ставка, установленная для родительской группы.
    pub fn use_parent_vat(&mut self, use_parent_vat: bool) -> &mut Self {
        let _ = self.use_parent_vat.insert(use_parent_vat);
        self
    }
    /// НДС %
    pub fn vat(&mut self, vat: i32) -> &mut Self {
        let _ = self.vat.insert(vat);
        self
    }
    /// Включен ли НДС для группы. С помощью этого флага для группы можно выставлять НДС = 0 или НДС = "без НДС". (vat = 0, vatEnabled = false) -> vat = "без НДС", (vat = 0, vatEnabled = true) -> vat = 0%.
    pub fn vat_enabled(&mut self, vat_enabled: bool) -> &mut Self {
        let _ = self.vat_enabled.insert(vat_enabled);
        self
    }
    pub fn build(&self) -> UpdateProductFolder {
        UpdateProductFolder {
            archived: self.archived,
            code: self.code.to_owned(),
            description: self.description.to_owned(),
            external_code: self.external_code.to_owned(),
            id: self.id,
            meta: self.meta.to_owned(),
            name: self.name.to_owned(),
            product_folder: self.product_folder.to_owned(),
            shared: self.shared,
            tax_system: self.tax_system.to_owned(),
            use_parent_vat: self.use_parent_vat,
            vat: self.vat,
            vat_enabled: self.vat_enabled,
        }
    }
}
