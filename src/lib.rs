//! # rust-moysklad
//!
//! `rust-moysklad` Библиотека для работы с API сервиса "Мой Склад".

mod api_client;
mod models;
pub use api_client::{FilterOperator, MoySkladApiClient};
pub use models::{
    assortment::Assortment,
    counterparty::{CompanyType, Counterparty, Sex},
    currency::Currency,
    product_folder::ProductFolder,
    uom::Uom,
    TaxSystem,
};
