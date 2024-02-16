use super::Meta;
use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Characteristic {
    pub meta: Option<Meta>,
    pub id: uuid::Uuid,
    pub name: String,
    pub value: String,
}
impl Characteristic {
    pub fn from_variant_char<T>(char: VariantCharacteristic, value: T) -> Self
    where
        T: ToString,
    {
        Self {
            meta: None,
            id: char.id,
            name: char.name,
            value: value.to_string(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariantCharacteristic {
    pub id: uuid::Uuid,
    pub meta: Meta,
    pub name: String,
    pub required: bool,
    #[serde(rename = "type")]
    pub characteristic_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharResponse {
    pub meta: Meta,
    pub characteristics: Vec<VariantCharacteristic>,
}
