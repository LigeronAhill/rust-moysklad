use crate::{
    api_client::MsEntity,
    models::{Meta, MetaWrapper},
    TaxSystem,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{
    assortment::{Barcode, BuyPrice, MinPrice, SalePrice},
    deserialize_option_date_from_str, Attribute,
};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub account_id: uuid::Uuid,
    pub archived: Option<bool>,
    pub article: Option<String>,
    pub attributes: Option<Vec<Attribute>>,
    pub barcodes: Option<Vec<Barcode>>,
    pub buy_price: Option<BuyPrice>,
    pub code: Option<String>,
    pub country: Option<MetaWrapper>,
    pub description: Option<String>,
    pub discount_prohibited: Option<bool>,
    pub effective_vat: Option<i64>,
    pub effective_vat_enabled: Option<bool>,
    pub external_code: Option<String>,
    pub files: Option<MetaWrapper>,
    pub group: Option<MetaWrapper>,
    pub id: uuid::Uuid,
    pub images: Option<MetaWrapper>,
    pub is_serial_trackable: Option<bool>,
    pub meta: Meta,
    pub min_price: Option<MinPrice>,
    pub minimum_balance: Option<i32>,
    pub name: Option<String>,
    pub owner: Option<MetaWrapper>,
    pub packs: Option<Vec<Pack>>,
    pub partial_disposal: Option<bool>,
    pub path_name: Option<String>,
    pub payment_item_type: Option<String>,
    pub product_folder: Option<MetaWrapper>,
    pub sale_prices: Vec<SalePrice>,
    pub shared: Option<bool>,
    pub supplier: Option<MetaWrapper>,
    pub tax_system: Option<TaxSystem>,
    pub things: Option<Vec<String>>,
    pub tnved: Option<String>,
    pub tracking_type: Option<String>,
    pub uom: Option<MetaWrapper>,
    #[serde(deserialize_with = "deserialize_option_date_from_str")]
    pub updated: Option<NaiveDateTime>,
    pub use_parent_vat: Option<bool>,
    pub variants_count: Option<i64>,
    pub vat: Option<i64>,
    pub vat_enabled: Option<bool>,
    pub volume: Option<f64>,
    pub weight: Option<f64>,
}
impl Product {
    pub fn create<T>(name: T) -> CreateProductBuilder
    where
        T: ToString,
    {
        CreateProductBuilder {
            name: name.to_string(),
            ..Default::default()
        }
    }
    pub fn update() -> UpdateProductBuilder {
        UpdateProductBuilder::default()
    }
}
impl MsEntity for Product {
    fn url() -> String {
        String::from("https://api.moysklad.ru/api/remap/1.2/entity/product")
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pack {
    pub barcodes: Option<Vec<String>>,
    pub id: uuid::Uuid,
    pub quantity: f64,
    pub uom: MetaWrapper,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct CreateProduct {
    #[serde(skip_serializing_if = "Option::is_none")]
    article: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<Vec<Attribute>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    barcodes: Option<Vec<Barcode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    buy_price: Option<BuyPrice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country: Option<MetaWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discount_prohibited: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    files: Option<MetaWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<MetaWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    images: Option<MetaWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_serial_trackable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_price: Option<MinPrice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_balance: Option<i32>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    packs: Option<Vec<Pack>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partial_disposal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_item_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_folder: Option<MetaWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sale_prices: Option<Vec<CreateSalePrice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shared: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supplier: Option<MetaWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_system: Option<TaxSystem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    things: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tnved: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tracking_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uom: Option<MetaWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_parent_vat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vat: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vat_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<f64>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSalePrice {
    pub value: f64,
    pub currency: MetaWrapper,
    pub price_type: MetaWrapper,
}

#[derive(Default)]
pub struct CreateProductBuilder {
    article: Option<String>,
    attributes: Option<Vec<Attribute>>,
    barcodes: Option<Vec<Barcode>>,
    buy_price: Option<BuyPrice>,
    code: Option<String>,
    country: Option<MetaWrapper>,
    description: Option<String>,
    discount_prohibited: Option<bool>,
    external_code: Option<String>,
    files: Option<MetaWrapper>,
    group: Option<MetaWrapper>,
    images: Option<MetaWrapper>,
    is_serial_trackable: Option<bool>,
    min_price: Option<MinPrice>,
    minimum_balance: Option<i32>,
    name: String,
    packs: Option<Vec<Pack>>,
    partial_disposal: Option<bool>,
    payment_item_type: Option<String>,
    product_folder: Option<MetaWrapper>,
    sale_prices: Option<Vec<CreateSalePrice>>,
    shared: Option<bool>,
    supplier: Option<MetaWrapper>,
    tax_system: Option<TaxSystem>,
    things: Option<Vec<String>>,
    tnved: Option<String>,
    tracking_type: Option<String>,
    uom: Option<MetaWrapper>,
    use_parent_vat: Option<bool>,
    vat: Option<i64>,
    vat_enabled: Option<bool>,
    volume: Option<f64>,
    weight: Option<f64>,
}
impl CreateProductBuilder {
    pub fn article<T>(&mut self, article: T) -> &mut Self
    where
        T: ToString,
    {
        let _ = self.article.insert(article.to_string());
        self
    }
    pub fn attribute(&mut self, attribute: Attribute) -> &mut Self {
        self.attributes.get_or_insert(vec![]).push(attribute);
        self
    }
    pub fn barcode<T>(&mut self, barcode: T) -> &mut Self
    where
        T: ToString,
    {
        self.barcodes.get_or_insert(vec![]).push(Barcode {
            ean13: barcode.to_string(),
        });
        self
    }
    pub fn buy_price(&mut self, value: f64, currency_meta: Meta) -> &mut Self {
        let _ = self.buy_price.insert(BuyPrice {
            value,
            currency: MetaWrapper {
                meta: currency_meta,
            },
        });
        self
    }
    pub fn code<T>(&mut self, code: T) -> &mut Self
    where
        T: ToString,
    {
        let _ = self.code.insert(code.to_string());
        self
    }
    pub fn country(&mut self, country_meta: &Meta) -> &mut Self {
        let _ = self.country.insert(MetaWrapper {
            meta: country_meta.to_owned(),
        });
        self
    }
    pub fn description<T>(&mut self, description: T) -> &mut Self
    where
        T: ToString,
    {
        let _ = self.description.insert(description.to_string());
        self
    }
    pub fn discount_prohibited(&mut self, discount_prohibited: bool) -> &mut Self {
        let _ = self.discount_prohibited.insert(discount_prohibited);
        self
    }
    pub fn external_code<T>(&mut self, external_code: T) -> &mut Self
    where
        T: ToString,
    {
        let _ = self.external_code.insert(external_code.to_string());
        self
    }
    pub fn files(&mut self, files_meta: Meta) -> &mut Self {
        let _ = self.files.insert(MetaWrapper { meta: files_meta });
        self
    }
    pub fn group(&mut self, group_meta: Meta) -> &mut Self {
        let _ = self.group.insert(MetaWrapper { meta: group_meta });
        self
    }
    pub fn images(&mut self, images_meta: Meta) -> &mut Self {
        let _ = self.images.insert(MetaWrapper { meta: images_meta });
        self
    }
    pub fn is_serial_trackable(&mut self, is_serial_trackable: bool) -> &mut Self {
        let _ = self.is_serial_trackable.insert(is_serial_trackable);
        self
    }
    pub fn min_price(&mut self, value: f64, currency_meta: Meta) -> &mut Self {
        let _ = self.min_price.insert(MinPrice {
            value,
            currency: MetaWrapper {
                meta: currency_meta,
            },
        });
        self
    }
    pub fn minimum_balance(&mut self, minimum_balance: i32) -> &mut Self {
        let _ = self.minimum_balance.insert(minimum_balance);
        self
    }
    pub fn pack(&mut self, pack: Pack) -> &mut Self {
        self.packs.get_or_insert(vec![]).push(pack);
        self
    }
    pub fn partial_disposal(&mut self, partial_disposal: bool) -> &mut Self {
        let _ = self.partial_disposal.insert(partial_disposal);
        self
    }
    pub fn payment_item_type<T>(&mut self, payment_item_type: T) -> &mut Self
    where
        T: ToString,
    {
        let _ = self.payment_item_type.insert(payment_item_type.to_string());
        self
    }
    pub fn product_folder(&mut self, product_folder_meta: &Meta) -> &mut Self {
        let _ = self.product_folder.insert(MetaWrapper {
            meta: product_folder_meta.to_owned(),
        });
        self
    }
    pub fn sale_price(
        &mut self,
        value: f64,
        currency_meta: &Meta,
        price_type_meta: &Meta,
    ) -> &mut Self {
        self.sale_prices
            .get_or_insert(vec![])
            .push(CreateSalePrice {
                value,
                currency: MetaWrapper {
                    meta: currency_meta.to_owned(),
                },
                price_type: MetaWrapper {
                    meta: price_type_meta.to_owned(),
                },
            });
        self
    }
    pub fn shared(&mut self, shared: bool) -> &mut Self {
        let _ = self.shared.insert(shared);
        self
    }
    pub fn supplier(&mut self, supplier_meta: Meta) -> &mut Self {
        let _ = self.supplier.insert(MetaWrapper {
            meta: supplier_meta,
        });
        self
    }
    pub fn tax_system(&mut self, tax_system: TaxSystem) -> &mut Self {
        let _ = self.tax_system.insert(tax_system);
        self
    }
    pub fn thing<T>(&mut self, thing: T) -> &mut Self
    where
        T: ToString,
    {
        self.things.get_or_insert(vec![]).push(thing.to_string());
        self
    }
    pub fn tnved<T>(&mut self, tnved: T) -> &mut Self
    where
        T: ToString,
    {
        let _ = self.tnved.insert(tnved.to_string());
        self
    }
    pub fn tracking_type<T>(&mut self, tracking_type: T) -> &mut Self
    where
        T: ToString,
    {
        let _ = self.tracking_type.insert(tracking_type.to_string());
        self
    }
    pub fn uom(&mut self, uom_meta: &Meta) -> &mut Self {
        let _ = self.uom.insert(MetaWrapper {
            meta: uom_meta.to_owned(),
        });
        self
    }
    pub fn use_parent_vat(&mut self, use_parent_vat: bool) -> &mut Self {
        let _ = self.use_parent_vat.insert(use_parent_vat);
        self
    }
    pub fn vat(&mut self, vat: i64) -> &mut Self {
        let _ = self.vat.insert(vat);
        self
    }
    pub fn vat_enabled(&mut self, vat_enabled: bool) -> &mut Self {
        let _ = self.vat_enabled.insert(vat_enabled);
        self
    }
    pub fn volume(&mut self, volume: f64) -> &mut Self {
        let _ = self.volume.insert(volume);
        self
    }
    pub fn weight(&mut self, weight: f64) -> &mut Self {
        let _ = self.weight.insert(weight);
        self
    }
    pub fn build(&self) -> CreateProduct {
        CreateProduct {
            article: self.article.to_owned(),
            attributes: self.attributes.to_owned(),
            barcodes: self.barcodes.to_owned(),
            buy_price: self.buy_price.to_owned(),
            code: self.code.to_owned(),
            country: self.country.to_owned(),
            description: self.description.to_owned(),
            discount_prohibited: self.discount_prohibited,
            external_code: self.external_code.to_owned(),
            files: self.files.to_owned(),
            group: self.group.to_owned(),
            images: self.images.to_owned(),
            is_serial_trackable: self.is_serial_trackable,
            min_price: self.min_price.to_owned(),
            minimum_balance: self.minimum_balance,
            name: self.name.to_owned(),
            packs: self.packs.to_owned(),
            partial_disposal: self.partial_disposal,
            payment_item_type: self.payment_item_type.to_owned(),
            product_folder: self.product_folder.to_owned(),
            sale_prices: self.sale_prices.to_owned(),
            shared: self.shared,
            supplier: self.supplier.to_owned(),
            tax_system: self.tax_system.to_owned(),
            things: self.things.to_owned(),
            tnved: self.tnved.to_owned(),
            tracking_type: self.tracking_type.to_owned(),
            uom: self.uom.to_owned(),
            use_parent_vat: self.use_parent_vat,
            vat: self.vat,
            vat_enabled: self.vat_enabled,
            volume: self.volume,
            weight: self.weight,
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct UpdateProduct {
    #[serde(skip_serializing_if = "Option::is_none")]
    article: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<Vec<Attribute>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    barcodes: Option<Vec<Barcode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    buy_price: Option<BuyPrice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country: Option<MetaWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discount_prohibited: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    files: Option<MetaWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<MetaWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    images: Option<MetaWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_serial_trackable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<Meta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_price: Option<MinPrice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_balance: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    packs: Option<Vec<Pack>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partial_disposal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_item_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_folder: Option<MetaWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sale_prices: Option<Vec<CreateSalePrice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shared: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supplier: Option<MetaWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_system: Option<TaxSystem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    things: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tnved: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tracking_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uom: Option<MetaWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_parent_vat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vat: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vat_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<f64>,
}
#[derive(Default)]
pub struct UpdateProductBuilder {
    article: Option<String>,
    attributes: Option<Vec<Attribute>>,
    barcodes: Option<Vec<Barcode>>,
    buy_price: Option<BuyPrice>,
    code: Option<String>,
    country: Option<MetaWrapper>,
    description: Option<String>,
    discount_prohibited: Option<bool>,
    external_code: Option<String>,
    files: Option<MetaWrapper>,
    group: Option<MetaWrapper>,
    images: Option<MetaWrapper>,
    is_serial_trackable: Option<bool>,
    meta: Option<Meta>,
    min_price: Option<MinPrice>,
    minimum_balance: Option<i32>,
    name: Option<String>,
    packs: Option<Vec<Pack>>,
    partial_disposal: Option<bool>,
    payment_item_type: Option<String>,
    product_folder: Option<MetaWrapper>,
    sale_prices: Option<Vec<CreateSalePrice>>,
    shared: Option<bool>,
    supplier: Option<MetaWrapper>,
    tax_system: Option<TaxSystem>,
    things: Option<Vec<String>>,
    tnved: Option<String>,
    tracking_type: Option<String>,
    uom: Option<MetaWrapper>,
    use_parent_vat: Option<bool>,
    vat: Option<i64>,
    vat_enabled: Option<bool>,
    volume: Option<f64>,
    weight: Option<f64>,
}
impl UpdateProductBuilder {
    pub fn article<T>(&mut self, article: T) -> &mut Self
    where
        T: ToString,
    {
        let _ = self.article.insert(article.to_string());
        self
    }
    pub fn attribute(&mut self, attribute: Attribute) -> &mut Self {
        self.attributes.get_or_insert(vec![]).push(attribute);
        self
    }
    pub fn barcode<T>(&mut self, barcode: T) -> &mut Self
    where
        T: ToString,
    {
        self.barcodes.get_or_insert(vec![]).push(Barcode {
            ean13: barcode.to_string(),
        });
        self
    }
    pub fn buy_price(&mut self, value: f64, currency_meta: Meta) -> &mut Self {
        let _ = self.buy_price.insert(BuyPrice {
            value,
            currency: MetaWrapper {
                meta: currency_meta,
            },
        });
        self
    }
    pub fn code<T>(&mut self, code: T) -> &mut Self
    where
        T: ToString,
    {
        let _ = self.code.insert(code.to_string());
        self
    }
    pub fn country(&mut self, country_meta: Meta) -> &mut Self {
        let _ = self.country.insert(MetaWrapper { meta: country_meta });
        self
    }
    pub fn description<T>(&mut self, description: T) -> &mut Self
    where
        T: ToString,
    {
        let _ = self.description.insert(description.to_string());
        self
    }
    pub fn discount_prohibited(&mut self, discount_prohibited: bool) -> &mut Self {
        let _ = self.discount_prohibited.insert(discount_prohibited);
        self
    }
    pub fn external_code<T>(&mut self, external_code: T) -> &mut Self
    where
        T: ToString,
    {
        let _ = self.external_code.insert(external_code.to_string());
        self
    }
    pub fn files(&mut self, files_meta: Meta) -> &mut Self {
        let _ = self.files.insert(MetaWrapper { meta: files_meta });
        self
    }
    pub fn group(&mut self, group_meta: Meta) -> &mut Self {
        let _ = self.group.insert(MetaWrapper { meta: group_meta });
        self
    }
    pub fn images(&mut self, images_meta: Meta) -> &mut Self {
        let _ = self.images.insert(MetaWrapper { meta: images_meta });
        self
    }
    pub fn is_serial_trackable(&mut self, is_serial_trackable: bool) -> &mut Self {
        let _ = self.is_serial_trackable.insert(is_serial_trackable);
        self
    }
    pub fn meta(&mut self, meta: Meta) -> &mut Self {
        let _ = self.meta.insert(meta);
        self
    }
    pub fn min_price(&mut self, value: f64, currency_meta: Meta) -> &mut Self {
        let _ = self.min_price.insert(MinPrice {
            value,
            currency: MetaWrapper {
                meta: currency_meta,
            },
        });
        self
    }
    pub fn minimum_balance(&mut self, minimum_balance: i32) -> &mut Self {
        let _ = self.minimum_balance.insert(minimum_balance);
        self
    }
    pub fn name<T>(&mut self, name: T) -> &mut Self
    where
        T: ToString,
    {
        let _ = self.name.insert(name.to_string());
        self
    }
    pub fn pack(&mut self, pack: Pack) -> &mut Self {
        self.packs.get_or_insert(vec![]).push(pack);

        self
    }
    pub fn partial_disposal(&mut self, partial_disposal: bool) -> &mut Self {
        let _ = self.partial_disposal.insert(partial_disposal);
        self
    }
    pub fn payment_item_type<T>(&mut self, payment_item_type: T) -> &mut Self
    where
        T: ToString,
    {
        let _ = self.payment_item_type.insert(payment_item_type.to_string());
        self
    }
    pub fn product_folder(&mut self, product_folder_meta: Meta) -> &mut Self {
        let _ = self.product_folder.insert(MetaWrapper {
            meta: product_folder_meta,
        });
        self
    }
    pub fn sale_price(
        &mut self,
        value: f64,
        currency_meta: Meta,
        price_type_meta: Meta,
    ) -> &mut Self {
        self.sale_prices
            .get_or_insert(vec![])
            .push(CreateSalePrice {
                value,
                currency: MetaWrapper {
                    meta: currency_meta,
                },
                price_type: MetaWrapper {
                    meta: price_type_meta,
                },
            });
        self
    }
    pub fn shared(&mut self, shared: bool) -> &mut Self {
        let _ = self.shared.insert(shared);
        self
    }
    pub fn supplier(&mut self, supplier_meta: Meta) -> &mut Self {
        let _ = self.supplier.insert(MetaWrapper {
            meta: supplier_meta,
        });
        self
    }
    pub fn tax_system(&mut self, tax_system: TaxSystem) -> &mut Self {
        let _ = self.tax_system.insert(tax_system);
        self
    }
    pub fn thing<T>(&mut self, thing: T) -> &mut Self
    where
        T: ToString,
    {
        self.things.get_or_insert(vec![]).push(thing.to_string());
        self
    }
    pub fn tnved<T>(&mut self, tnved: T) -> &mut Self
    where
        T: ToString,
    {
        let _ = self.tnved.insert(tnved.to_string());
        self
    }
    pub fn tracking_type<T>(&mut self, tracking_type: T) -> &mut Self
    where
        T: ToString,
    {
        let _ = self.tracking_type.insert(tracking_type.to_string());
        self
    }
    pub fn uom(&mut self, uom_meta: Meta) -> &mut Self {
        let _ = self.uom.insert(MetaWrapper { meta: uom_meta });
        self
    }
    pub fn use_parent_vat(&mut self, use_parent_vat: bool) -> &mut Self {
        let _ = self.use_parent_vat.insert(use_parent_vat);
        self
    }
    pub fn vat(&mut self, vat: i64) -> &mut Self {
        let _ = self.vat.insert(vat);
        self
    }
    pub fn vat_enabled(&mut self, vat_enabled: bool) -> &mut Self {
        let _ = self.vat_enabled.insert(vat_enabled);
        self
    }
    pub fn volume(&mut self, volume: f64) -> &mut Self {
        let _ = self.volume.insert(volume);
        self
    }
    pub fn weight(&mut self, weight: f64) -> &mut Self {
        let _ = self.weight.insert(weight);
        self
    }
    pub fn build(&self) -> UpdateProduct {
        UpdateProduct {
            article: self.article.to_owned(),
            attributes: self.attributes.to_owned(),
            barcodes: self.barcodes.to_owned(),
            buy_price: self.buy_price.to_owned(),
            code: self.code.to_owned(),
            country: self.country.to_owned(),
            description: self.description.to_owned(),
            discount_prohibited: self.discount_prohibited,
            external_code: self.external_code.to_owned(),
            files: self.files.to_owned(),
            group: self.group.to_owned(),
            images: self.images.to_owned(),
            is_serial_trackable: self.is_serial_trackable,
            min_price: self.min_price.to_owned(),
            minimum_balance: self.minimum_balance,
            name: self.name.to_owned(),
            packs: self.packs.to_owned(),
            partial_disposal: self.partial_disposal,
            payment_item_type: self.payment_item_type.to_owned(),
            product_folder: self.product_folder.to_owned(),
            sale_prices: self.sale_prices.to_owned(),
            shared: self.shared,
            supplier: self.supplier.to_owned(),
            tax_system: self.tax_system.to_owned(),
            things: self.things.to_owned(),
            tnved: self.tnved.to_owned(),
            tracking_type: self.tracking_type.to_owned(),
            uom: self.uom.to_owned(),
            use_parent_vat: self.use_parent_vat,
            vat: self.vat,
            vat_enabled: self.vat_enabled,
            volume: self.volume,
            weight: self.weight,
            meta: self.meta.to_owned(),
        }
    }
}
