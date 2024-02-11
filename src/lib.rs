//! # rust-moysklad
//!
//! `rust-moysklad` Библиотека для работы с API сервиса "Мой Склад".

mod api_client;
mod models;
pub use api_client::MoySkladApiClient;
pub use models::assortment::Assortment;
