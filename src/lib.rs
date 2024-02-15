//! # rust-moysklad
//!
//! `rust-moysklad` Библиотека для работы с API сервиса "Мой Склад".

mod api_client;
mod models;
pub use api_client::{FilterOperator, MoySkladApiClient};
pub use models::{
    assortment::Assortment,
    counterparty::{CompanyType, Counterparty, Sex},
    country::Country,
    currency::Currency,
    product::Product,
    product_folder::ProductFolder,
    region::Region,
    uom::Uom,
    Attribute, AttributeValue, CustomValue, PriceType, ProductsCustomField, TaxSystem,
};
