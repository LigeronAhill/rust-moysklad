use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::api_client::MsEntity;

use super::{deserialize_date_from_str, Attribute, Meta, MetaWrapper, PriceType};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Counterparty {
    /// ID учетной записи
    pub account_id: uuid::Uuid,
    /// Массив счетов Контрагентов
    pub accounts: MetaWrapper,
    /// Фактический адрес Контрагента
    pub actual_address: Option<String>,
    /// Фактический адрес Контрагента с детализацией по отдельным полям
    pub actual_address_full: Option<Address>,
    /// Добавлен ли Контрагент в архив
    pub archived: bool,
    /// Операторы доп. полей. Массив метаданных доп. полей
    pub attributes: Option<Vec<Attribute>>,
    /// Бонусные баллы по активной бонусной программе
    pub bonus_points: Option<i32>,
    /// Метаданные активной Бонусной программы
    pub bonus_program: Option<MetaWrapper>,
    /// Код Контрагента
    pub code: Option<String>,
    /// Тип Контрагента.
    pub company_type: CompanyType,
    /// Момент создания
    #[serde(deserialize_with = "deserialize_date_from_str")]
    pub created: NaiveDateTime,
    /// Комментарий к Контрагенту
    pub description: Option<String>,
    /// Номер дисконтной карты Контрагента
    pub discount_card_number: Option<String>,
    /// Массив скидок Контрагента. Массив может содержать персональные и накопительные скидки. Персональная скидка выводится, если хотя бы раз изменялся процент скидки для контрагента, значение будет указано в поле personalDiscount
    pub discounts: Option<MetaWrapper>,
    /// Адрес электронной почты
    pub email: Option<String>,
    /// Внешний код Контрагента
    pub external_code: String,
    /// Номер факса
    pub fax: Option<String>,
    /// Метаданные массива Файлов (Максимальное количество файлов - 100)
    pub files: MetaWrapper,
    /// Отдел сотрудника
    pub group: MetaWrapper,
    /// ID Контрагента
    pub id: uuid::Uuid,
    /// Метаданные Контрагента
    pub meta: Meta,
    /// Наименование Контрагента
    pub name: String,
    /// Массив событий Контрагента. Подробнее тут
    pub notes: MetaWrapper,
    /// Владелец (Сотрудник)
    pub owner: Option<MetaWrapper>,
    /// Номер городского телефона
    pub phone: Option<String>,
    /// Тип цены Контрагента. Подробнее тут
    pub price_type: Option<PriceType>,
    /// Сумма продаж
    pub sales_amount: f64,
    /// Общий доступ
    pub shared: bool,
    /// Метаданные Статуса Контрагента
    pub state: Option<MetaWrapper>,
    /// ID синхронизации
    pub sync_id: Option<uuid::Uuid>,
    /// Группы контрагента
    pub tags: Vec<String>,
    /// Момент последнего обновления Контрагента
    #[serde(deserialize_with = "deserialize_date_from_str")]
    pub updated: NaiveDateTime,
    /// Полное наименование Контрагента
    pub legal_title: Option<String>,
    /// Юридического адреса Контрагента
    pub legal_address: Option<String>,
    /// Юридический адрес Контрагента с детализацией по отдельным полям
    pub legal_address_full: Option<Address>,
    /// ИНН
    pub inn: Option<String>,
    /// КПП
    pub kpp: Option<String>,
    /// ОГРН
    pub ogrn: Option<String>,
    /// ОКПО
    pub okpo: Option<String>,
    /// ОКПО
    pub ogrnip: Option<String>,
    /// Фамилия для Контрагента типа [Индивидуальный предприниматель, Физическое лицо]. Игнорируется для Контрагентов типа [Юридическое лицо]
    pub legal_last_name: Option<String>,
    /// Имя для Контрагента типа [Индивидуальный предприниматель, Физическое лицо]. Игнорируется для Контрагентов типа [Юридическое лицо]
    pub legal_first_name: Option<String>,
    /// Отчество для Контрагента типа [Индивидуальный предприниматель, Физическое лицо]. Игнорируется для Контрагентов типа [Юридическое лицо]
    pub legal_middle_name: Option<String>,
    /// Пол Контрагента
    pub sex: Option<Sex>,
    /// Номер свидетельства
    pub certificate_number: Option<String>,
    /// Дата свидетельства
    pub certificate_date: Option<String>,
}
/// Адрес
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    /// Другое
    pub add_info: Option<String>,
    /// Почтовый индекс
    pub postal_code: Option<String>,
    /// Метаданные страны
    pub country: Option<MetaWrapper>,
    /// Метаданные региона
    pub region: Option<MetaWrapper>,
    /// Город
    pub city: Option<String>,
    /// Улица
    pub street: Option<String>,
    /// Дом
    pub house: Option<String>,
    /// Квартира
    pub apartment: Option<String>,
    /// Комментарий
    pub comment: Option<String>,
}
impl Counterparty {
    pub fn create(name: impl Into<String>) -> CreateCounterpartyBuilder {
        CreateCounterpartyBuilder::new(name)
    }
    pub fn update() -> UpdateCounterpartyBuilder {
        UpdateCounterpartyBuilder::default()
    }
}
/// Тип Контрагента
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CompanyType {
    /// Юридическое лицо
    Legal,
    /// Индивидуальный предприниматель
    Entrepreneur,
    /// Физическое лицо
    #[default]
    Individual,
}
/// Пол Контрагента
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Sex {
    /// Мужской
    #[default]
    Male,
    /// Женский
    Female,
}
impl MsEntity for Counterparty {
    fn url() -> String {
        String::from("https://api.moysklad.ru/api/remap/1.2/entity/counterparty")
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCounterparty {
    #[serde(skip_serializing_if = "Option::is_none")]
    actual_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    actual_address_full: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<Vec<Attribute>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bonus_program: Option<MetaWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    company_type: Option<CompanyType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discount_card_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fax: Option<String>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_type: Option<MetaWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shared: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<MetaWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    legal_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    legal_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    legal_address_full: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kpp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ogrn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    okpo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ogrnip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    legal_last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    legal_first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    legal_middle_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sex: Option<Sex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_date: Option<String>,
}
#[derive(Default)]
pub struct CreateCounterpartyBuilder {
    actual_address: Option<String>,
    actual_address_full: Option<Address>,
    attributes: Option<Vec<Attribute>>,
    bonus_program: Option<MetaWrapper>,
    company_type: Option<CompanyType>,
    description: Option<String>,
    discount_card_number: Option<String>,
    email: Option<String>,
    external_code: Option<String>,
    fax: Option<String>,
    name: String,
    phone: Option<String>,
    price_type: Option<MetaWrapper>,
    shared: Option<bool>,
    state: Option<MetaWrapper>,
    tags: Option<Vec<String>>,
    legal_title: Option<String>,
    legal_address: Option<String>,
    legal_address_full: Option<Address>,
    inn: Option<String>,
    kpp: Option<String>,
    ogrn: Option<String>,
    okpo: Option<String>,
    ogrnip: Option<String>,
    legal_last_name: Option<String>,
    legal_first_name: Option<String>,
    legal_middle_name: Option<String>,
    sex: Option<Sex>,
    certificate_number: Option<String>,
    certificate_date: Option<String>,
}
impl CreateCounterpartyBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }
    /// Фактический адрес Контрагента
    pub fn actual_address(&mut self, actual_address: impl Into<String>) -> &mut Self {
        let _ = self.actual_address.insert(actual_address.into());
        self
    }
    pub fn actual_address_full_add_info(&mut self, add_info: impl Into<String>) -> &mut Self {
        let _ = self
            .actual_address_full
            .get_or_insert(Address::default())
            .add_info
            .insert(add_info.into());
        self
    }
    pub fn actual_address_full_postal_code(&mut self, postal_code: impl Into<String>) -> &mut Self {
        let _ = self
            .actual_address_full
            .get_or_insert(Address::default())
            .postal_code
            .insert(postal_code.into());
        self
    }
    pub fn actual_address_full_country(&mut self, country_meta: Meta) -> &mut Self {
        let _ = self
            .actual_address_full
            .get_or_insert(Address::default())
            .country
            .insert(MetaWrapper { meta: country_meta });
        self
    }
    pub fn actual_address_full_region(&mut self, region_meta: Meta) -> &mut Self {
        let _ = self
            .actual_address_full
            .get_or_insert(Address::default())
            .region
            .insert(MetaWrapper { meta: region_meta });
        self
    }
    pub fn actual_address_full_city(&mut self, city: impl Into<String>) -> &mut Self {
        let _ = self
            .actual_address_full
            .get_or_insert(Address::default())
            .city
            .insert(city.into());
        self
    }
    pub fn actual_address_full_street(&mut self, street: impl Into<String>) -> &mut Self {
        let _ = self
            .actual_address_full
            .get_or_insert(Address::default())
            .street
            .insert(street.into());
        self
    }
    pub fn actual_address_full_house(&mut self, house: impl Into<String>) -> &mut Self {
        let _ = self
            .actual_address_full
            .get_or_insert(Address::default())
            .house
            .insert(house.into());
        self
    }
    pub fn actual_address_full_apartment(&mut self, apartment: impl Into<String>) -> &mut Self {
        let _ = self
            .actual_address_full
            .get_or_insert(Address::default())
            .apartment
            .insert(apartment.into());
        self
    }
    pub fn actual_address_full_comment(&mut self, comment: impl Into<String>) -> &mut Self {
        let _ = self
            .actual_address_full
            .get_or_insert(Address::default())
            .comment
            .insert(comment.into());
        self
    }
    /// Операторы доп. полей. Массив метаданных доп. полей
    pub fn attribute(&mut self, attribute: Attribute) -> &mut Self {
        self.attributes.get_or_insert(vec![]).push(attribute);
        self
    }
    /// Метаданные активной Бонусной программы
    pub fn bonus_program(&mut self, bonus_programm_meta: Meta) -> &mut Self {
        let _ = self.bonus_program.insert(MetaWrapper {
            meta: bonus_programm_meta,
        });
        self
    }
    /// Тип Контрагента.
    pub fn company_type(&mut self, company_type: CompanyType) -> &mut Self {
        let _ = self.company_type.insert(company_type);
        self
    }
    /// Комментарий к Контрагенту
    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        let _ = self.description.insert(description.into());
        self
    }
    /// Номер дисконтной карты Контрагента
    pub fn discount_card_number(&mut self, discount_card_number: impl Into<String>) -> &mut Self {
        let _ = self
            .discount_card_number
            .insert(discount_card_number.into());
        self
    }
    /// Адрес электронной почты
    pub fn email(&mut self, email: impl Into<String>) -> &mut Self {
        let _ = self.email.insert(email.into());
        self
    }
    /// Внешний код Контрагента
    pub fn external_code<T>(&mut self, external_code: T) -> &mut Self
    where
        T: ToString,
    {
        let _ = self.external_code.insert(external_code.to_string());
        self
    }
    /// Номер факса
    pub fn fax(&mut self, fax: impl Into<String>) -> &mut Self {
        let _ = self.fax.insert(fax.into());
        self
    }
    /// Номер городского телефона
    pub fn phone(&mut self, phone: impl Into<String>) -> &mut Self {
        let _ = self.phone.insert(phone.into());
        self
    }
    /// Тип цены Контрагента. Подробнее тут
    pub fn price_type(&mut self, price_type_meta: Meta) -> &mut Self {
        let _ = self.price_type.insert(MetaWrapper {
            meta: price_type_meta,
        });
        self
    }
    /// Общий доступ
    pub fn shared(&mut self, shared: bool) -> &mut Self {
        let _ = self.shared.insert(shared);
        self
    }
    /// Метаданные Статуса Контрагента
    pub fn state(&mut self, state_meta: Meta) -> &mut Self {
        let _ = self.state.insert(MetaWrapper { meta: state_meta });
        self
    }
    /// Группы контрагента
    pub fn tag(&mut self, tag: impl Into<String>) -> &mut Self {
        self.tags.get_or_insert(vec![]).push(tag.into());
        self
    }
    /// Полное наименование Контрагента
    pub fn legal_title(&mut self, legal_title: impl Into<String>) -> &mut Self {
        let _ = self.legal_title.insert(legal_title.into());
        self
    }
    /// Юридического адреса Контрагента
    pub fn legal_address(&mut self, legal_address: impl Into<String>) -> &mut Self {
        let _ = self.legal_address.insert(legal_address.into());
        self
    }
    /// Юридический адрес Контрагента с детализацией по отдельным полям
    pub fn legal_address_full_add_info(&mut self, add_info: impl Into<String>) -> &mut Self {
        let _ = self
            .legal_address_full
            .get_or_insert(Address::default())
            .add_info
            .insert(add_info.into());
        self
    }
    pub fn legal_address_full_postal_code(&mut self, postal_code: impl Into<String>) -> &mut Self {
        let _ = self
            .legal_address_full
            .get_or_insert(Address::default())
            .postal_code
            .insert(postal_code.into());
        self
    }
    pub fn legal_address_full_country(&mut self, country_meta: Meta) -> &mut Self {
        let _ = self
            .legal_address_full
            .get_or_insert(Address::default())
            .country
            .insert(MetaWrapper { meta: country_meta });
        self
    }
    pub fn legal_address_full_region(&mut self, region_meta: Meta) -> &mut Self {
        let _ = self
            .legal_address_full
            .get_or_insert(Address::default())
            .region
            .insert(MetaWrapper { meta: region_meta });
        self
    }
    pub fn legal_address_full_city(&mut self, city: impl Into<String>) -> &mut Self {
        let _ = self
            .legal_address_full
            .get_or_insert(Address::default())
            .city
            .insert(city.into());
        self
    }
    pub fn legal_address_full_street(&mut self, street: impl Into<String>) -> &mut Self {
        let _ = self
            .legal_address_full
            .get_or_insert(Address::default())
            .street
            .insert(street.into());
        self
    }
    pub fn legal_address_full_house(&mut self, house: impl Into<String>) -> &mut Self {
        let _ = self
            .legal_address_full
            .get_or_insert(Address::default())
            .house
            .insert(house.into());
        self
    }
    pub fn legal_address_full_apartmen(&mut self, apartment: impl Into<String>) -> &mut Self {
        let _ = self
            .legal_address_full
            .get_or_insert(Address::default())
            .apartment
            .insert(apartment.into());
        self
    }
    pub fn legal_address_full_comment(&mut self, comment: impl Into<String>) -> &mut Self {
        let _ = self
            .legal_address_full
            .get_or_insert(Address::default())
            .comment
            .insert(comment.into());
        self
    }
    /// ИНН
    pub fn inn(&mut self, inn: impl Into<String>) -> &mut Self {
        let _ = self.inn.insert(inn.into());
        self
    }
    /// КПП
    pub fn kpp(&mut self, kpp: impl Into<String>) -> &mut Self {
        let _ = self.kpp.insert(kpp.into());
        self
    }
    /// ОГРН
    pub fn ogrn(&mut self, ogrn: impl Into<String>) -> &mut Self {
        let _ = self.ogrn.insert(ogrn.into());
        self
    }
    /// ОКПО
    pub fn okpo(&mut self, okpo: impl Into<String>) -> &mut Self {
        let _ = self.okpo.insert(okpo.into());
        self
    }
    /// ОКПО
    pub fn ogrnip(&mut self, ogrnip: impl Into<String>) -> &mut Self {
        let _ = self.ogrnip.insert(ogrnip.into());
        self
    }
    /// Фамилия для Контрагента типа [Индивидуальный предприниматель, Физическое лицо]. Игнорируется для Контрагентов типа [Юридическое лицо]
    pub fn legal_last_name(&mut self, legal_last_name: impl Into<String>) -> &mut Self {
        let _ = self.legal_last_name.insert(legal_last_name.into());
        self
    }
    /// Имя для Контрагента типа [Индивидуальный предприниматель, Физическое лицо]. Игнорируется для Контрагентов типа [Юридическое лицо]
    pub fn legal_first_name(&mut self, legal_first_name: impl Into<String>) -> &mut Self {
        let _ = self.legal_first_name.insert(legal_first_name.into());
        self
    }
    /// Отчество для Контрагента типа [Индивидуальный предприниматель, Физическое лицо]. Игнорируется для Контрагентов типа [Юридическое лицо]
    pub fn legal_middle_name(&mut self, legal_middle_name: impl Into<String>) -> &mut Self {
        let _ = self.legal_middle_name.insert(legal_middle_name.into());
        self
    }
    /// Пол Контрагента
    pub fn sex(&mut self, sex: Sex) -> &mut Self {
        let _ = self.sex.insert(sex);
        self
    }
    /// Номер свидетельства
    pub fn certificate_number(&mut self, certificate_number: impl Into<String>) -> &mut Self {
        let _ = self.certificate_number.insert(certificate_number.into());
        self
    }
    /// Дата свидетельства
    pub fn certificate_date(&mut self, certificate_date: impl Into<String>) -> &mut Self {
        let _ = self.certificate_date.insert(certificate_date.into());
        self
    }
    pub fn build(&self) -> CreateCounterparty {
        CreateCounterparty {
            actual_address: self.actual_address.to_owned(),
            actual_address_full: self.actual_address_full.to_owned(),
            attributes: self.attributes.to_owned(),
            bonus_program: self.bonus_program.to_owned(),
            company_type: self.company_type.to_owned(),
            description: self.description.to_owned(),
            discount_card_number: self.discount_card_number.to_owned(),
            email: self.email.to_owned(),
            external_code: self.external_code.to_owned(),
            fax: self.fax.to_owned(),
            name: self.name.to_owned(),
            phone: self.phone.to_owned(),
            price_type: self.price_type.to_owned(),
            shared: self.shared,
            state: self.state.to_owned(),
            tags: self.tags.to_owned(),
            legal_title: self.legal_title.to_owned(),
            legal_address: self.legal_address.to_owned(),
            legal_address_full: self.legal_address_full.to_owned(),
            inn: self.inn.to_owned(),
            kpp: self.kpp.to_owned(),
            ogrn: self.ogrn.to_owned(),
            okpo: self.okpo.to_owned(),
            ogrnip: self.ogrnip.to_owned(),
            legal_last_name: self.legal_last_name.to_owned(),
            legal_first_name: self.legal_first_name.to_owned(),
            legal_middle_name: self.legal_middle_name.to_owned(),
            sex: self.sex.to_owned(),
            certificate_number: self.certificate_number.to_owned(),
            certificate_date: self.certificate_date.to_owned(),
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCounterparty {
    #[serde(skip_serializing_if = "Option::is_none")]
    actual_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    actual_address_full: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    archived: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<Vec<Attribute>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bonus_program: Option<MetaWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    company_type: Option<CompanyType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discount_card_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fax: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<Meta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_type: Option<MetaWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shared: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<MetaWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    legal_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    legal_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    legal_address_full: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kpp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ogrn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    okpo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ogrnip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    legal_last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    legal_first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    legal_middle_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sex: Option<Sex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_date: Option<String>,
}
#[derive(Default)]
pub struct UpdateCounterpartyBuilder {
    actual_address: Option<String>,
    actual_address_full: Option<Address>,
    archived: Option<bool>,
    attributes: Option<Vec<Attribute>>,
    bonus_program: Option<MetaWrapper>,
    code: Option<String>,
    company_type: Option<CompanyType>,
    description: Option<String>,
    discount_card_number: Option<String>,
    email: Option<String>,
    external_code: Option<String>,
    fax: Option<String>,
    meta: Option<Meta>,
    name: Option<String>,
    phone: Option<String>,
    price_type: Option<MetaWrapper>,
    shared: Option<bool>,
    state: Option<MetaWrapper>,
    tags: Option<Vec<String>>,
    legal_title: Option<String>,
    legal_address: Option<String>,
    legal_address_full: Option<Address>,
    inn: Option<String>,
    kpp: Option<String>,
    ogrn: Option<String>,
    okpo: Option<String>,
    ogrnip: Option<String>,
    legal_last_name: Option<String>,
    legal_first_name: Option<String>,
    legal_middle_name: Option<String>,
    sex: Option<Sex>,
    certificate_number: Option<String>,
    certificate_date: Option<String>,
}
impl UpdateCounterpartyBuilder {
    /// Фактический адрес Контрагента
    pub fn actual_address(&mut self, actual_address: impl Into<String>) -> &mut Self {
        let _ = self.actual_address.insert(actual_address.into());
        self
    }
    /// Фактический адрес Контрагента с детализацией по отдельным полям
    pub fn actual_address_full_add_info(&mut self, add_info: impl Into<String>) -> &mut Self {
        let _ = self
            .actual_address_full
            .get_or_insert(Address::default())
            .add_info
            .insert(add_info.into());
        self
    }
    pub fn actual_address_full_postal_code(&mut self, postal_code: impl Into<String>) -> &mut Self {
        let _ = self
            .actual_address_full
            .get_or_insert(Address::default())
            .postal_code
            .insert(postal_code.into());
        self
    }
    pub fn actual_address_full_country(&mut self, country_meta: Meta) -> &mut Self {
        let _ = self
            .actual_address_full
            .get_or_insert(Address::default())
            .country
            .insert(MetaWrapper { meta: country_meta });
        self
    }
    pub fn actual_address_full_region(&mut self, region_meta: Meta) -> &mut Self {
        let _ = self
            .actual_address_full
            .get_or_insert(Address::default())
            .region
            .insert(MetaWrapper { meta: region_meta });
        self
    }
    pub fn actual_address_full_city(&mut self, city: impl Into<String>) -> &mut Self {
        let _ = self
            .actual_address_full
            .get_or_insert(Address::default())
            .city
            .insert(city.into());
        self
    }
    pub fn actual_address_full_street(&mut self, street: impl Into<String>) -> &mut Self {
        let _ = self
            .actual_address_full
            .get_or_insert(Address::default())
            .street
            .insert(street.into());
        self
    }
    pub fn actual_address_full_house(&mut self, house: impl Into<String>) -> &mut Self {
        let _ = self
            .actual_address_full
            .get_or_insert(Address::default())
            .house
            .insert(house.into());
        self
    }
    pub fn actual_address_full_apartment(&mut self, apartment: impl Into<String>) -> &mut Self {
        let _ = self
            .actual_address_full
            .get_or_insert(Address::default())
            .apartment
            .insert(apartment.into());
        self
    }
    pub fn actual_address_full_comment(&mut self, comment: impl Into<String>) -> &mut Self {
        let _ = self
            .actual_address_full
            .get_or_insert(Address::default())
            .comment
            .insert(comment.into());
        self
    }
    /// Добавлен ли Контрагент в архив
    pub fn archived(&mut self, archived: bool) -> &mut Self {
        let _ = self.archived.insert(archived);
        self
    }
    /// Операторы доп. полей. Массив метаданных доп. полей
    pub fn attribute(&mut self, attribute: Attribute) -> &mut Self {
        self.attributes.get_or_insert(vec![]).push(attribute);
        self
    }
    /// Метаданные активной Бонусной программы
    pub fn bonus_program(&mut self, bonus_programm_meta: Meta) -> &mut Self {
        let _ = self.bonus_program.insert(MetaWrapper {
            meta: bonus_programm_meta,
        });
        self
    }
    /// Тип Контрагента.
    pub fn company_type(&mut self, company_type: CompanyType) -> &mut Self {
        let _ = self.company_type.insert(company_type);
        self
    }
    /// Комментарий к Контрагенту
    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        let _ = self.description.insert(description.into());
        self
    }
    /// Код Контрагента
    pub fn code<T>(&mut self, code: T) -> &mut Self
    where
        T: ToString,
    {
        let _ = self.code.insert(code.to_string());
        self
    }
    /// Метаданные Контрагента
    pub fn meta(&mut self, meta: Meta) -> &mut Self {
        let _ = self.meta.insert(meta);
        self
    }
    /// Наименование Контрагента
    pub fn name<T>(&mut self, name: T) -> &mut Self
    where
        T: ToString,
    {
        let _ = self.name.insert(name.to_string());
        self
    }
    /// Номер дисконтной карты Контрагента
    pub fn discount_card_number(&mut self, discount_card_number: impl Into<String>) -> &mut Self {
        let _ = self
            .discount_card_number
            .insert(discount_card_number.into());
        self
    }
    /// Адрес электронной почты
    pub fn email(&mut self, email: impl Into<String>) -> &mut Self {
        let _ = self.email.insert(email.into());
        self
    }
    /// Внешний код Контрагента
    pub fn external_code<T>(&mut self, external_code: T) -> &mut Self
    where
        T: ToString,
    {
        let _ = self.external_code.insert(external_code.to_string());
        self
    }
    /// Номер факса
    pub fn fax(&mut self, fax: impl Into<String>) -> &mut Self {
        let _ = self.fax.insert(fax.into());
        self
    }
    /// Номер городского телефона
    pub fn phone(&mut self, phone: impl Into<String>) -> &mut Self {
        let _ = self.phone.insert(phone.into());
        self
    }
    /// Тип цены Контрагента. Подробнее тут
    pub fn price_type(&mut self, price_type_meta: Meta) -> &mut Self {
        let _ = self.price_type.insert(MetaWrapper {
            meta: price_type_meta,
        });
        self
    }
    /// Общий доступ
    pub fn shared(&mut self, shared: bool) -> &mut Self {
        let _ = self.shared.insert(shared);
        self
    }
    /// Метаданные Статуса Контрагента
    pub fn state(&mut self, state_meta: Meta) -> &mut Self {
        let _ = self.state.insert(MetaWrapper { meta: state_meta });
        self
    }
    /// Группы контрагента
    pub fn tag(&mut self, tag: impl Into<String>) -> &mut Self {
        self.tags.get_or_insert(vec![]).push(tag.into());
        self
    }
    /// Полное наименование Контрагента
    pub fn legal_title(&mut self, legal_title: impl Into<String>) -> &mut Self {
        let _ = self.legal_title.insert(legal_title.into());
        self
    }
    /// Юридического адреса Контрагента
    pub fn legal_address(&mut self, legal_address: impl Into<String>) -> &mut Self {
        let _ = self.legal_address.insert(legal_address.into());
        self
    }
    /// Юридический адрес Контрагента с детализацией по отдельным полям
    pub fn legal_address_full_add_info(&mut self, add_info: impl Into<String>) -> &mut Self {
        let _ = self
            .legal_address_full
            .get_or_insert(Address::default())
            .add_info
            .insert(add_info.into());
        self
    }
    pub fn legal_address_full_postal_code(&mut self, postal_code: impl Into<String>) -> &mut Self {
        let _ = self
            .legal_address_full
            .get_or_insert(Address::default())
            .postal_code
            .insert(postal_code.into());
        self
    }
    pub fn legal_address_full_country(&mut self, country_meta: Meta) -> &mut Self {
        let _ = self
            .legal_address_full
            .get_or_insert(Address::default())
            .country
            .insert(MetaWrapper { meta: country_meta });
        self
    }
    pub fn legal_address_full_region(&mut self, region_meta: Meta) -> &mut Self {
        let _ = self
            .legal_address_full
            .get_or_insert(Address::default())
            .region
            .insert(MetaWrapper { meta: region_meta });
        self
    }
    pub fn legal_address_full_city(&mut self, city: impl Into<String>) -> &mut Self {
        let _ = self
            .legal_address_full
            .get_or_insert(Address::default())
            .city
            .insert(city.into());
        self
    }
    pub fn legal_address_full_street(&mut self, street: impl Into<String>) -> &mut Self {
        let _ = self
            .legal_address_full
            .get_or_insert(Address::default())
            .street
            .insert(street.into());
        self
    }
    pub fn legal_address_full_house(&mut self, house: impl Into<String>) -> &mut Self {
        let _ = self
            .legal_address_full
            .get_or_insert(Address::default())
            .house
            .insert(house.into());
        self
    }
    pub fn legal_address_full_apartmen(&mut self, apartment: impl Into<String>) -> &mut Self {
        let _ = self
            .legal_address_full
            .get_or_insert(Address::default())
            .apartment
            .insert(apartment.into());
        self
    }
    pub fn legal_address_full_comment(&mut self, comment: impl Into<String>) -> &mut Self {
        let _ = self
            .legal_address_full
            .get_or_insert(Address::default())
            .comment
            .insert(comment.into());
        self
    }
    /// ИНН
    pub fn inn(&mut self, inn: impl Into<String>) -> &mut Self {
        let _ = self.inn.insert(inn.into());
        self
    }
    /// КПП
    pub fn kpp(&mut self, kpp: impl Into<String>) -> &mut Self {
        let _ = self.kpp.insert(kpp.into());
        self
    }
    /// ОГРН
    pub fn ogrn(&mut self, ogrn: impl Into<String>) -> &mut Self {
        let _ = self.ogrn.insert(ogrn.into());
        self
    }
    /// ОКПО
    pub fn okpo(&mut self, okpo: impl Into<String>) -> &mut Self {
        let _ = self.okpo.insert(okpo.into());
        self
    }
    /// ОКПО
    pub fn ogrnip(&mut self, ogrnip: impl Into<String>) -> &mut Self {
        let _ = self.ogrnip.insert(ogrnip.into());
        self
    }
    /// Фамилия для Контрагента типа [Индивидуальный предприниматель, Физическое лицо]. Игнорируется для Контрагентов типа [Юридическое лицо]
    pub fn legal_last_name(&mut self, legal_last_name: impl Into<String>) -> &mut Self {
        let _ = self.legal_last_name.insert(legal_last_name.into());
        self
    }
    /// Имя для Контрагента типа [Индивидуальный предприниматель, Физическое лицо]. Игнорируется для Контрагентов типа [Юридическое лицо]
    pub fn legal_first_name(&mut self, legal_first_name: impl Into<String>) -> &mut Self {
        let _ = self.legal_first_name.insert(legal_first_name.into());
        self
    }
    /// Отчество для Контрагента типа [Индивидуальный предприниматель, Физическое лицо]. Игнорируется для Контрагентов типа [Юридическое лицо]
    pub fn legal_middle_name(&mut self, legal_middle_name: impl Into<String>) -> &mut Self {
        let _ = self.legal_middle_name.insert(legal_middle_name.into());
        self
    }
    /// Пол Контрагента
    pub fn sex(&mut self, sex: Sex) -> &mut Self {
        let _ = self.sex.insert(sex);
        self
    }
    /// Номер свидетельства
    pub fn certificate_number(&mut self, certificate_number: impl Into<String>) -> &mut Self {
        let _ = self.certificate_number.insert(certificate_number.into());
        self
    }
    /// Дата свидетельства
    pub fn certificate_date(&mut self, certificate_date: impl Into<String>) -> &mut Self {
        let _ = self.certificate_date.insert(certificate_date.into());
        self
    }
    pub fn build(&self) -> UpdateCounterparty {
        UpdateCounterparty {
            actual_address: self.actual_address.to_owned(),
            actual_address_full: self.actual_address_full.to_owned(),
            attributes: self.attributes.to_owned(),
            bonus_program: self.bonus_program.to_owned(),
            company_type: self.company_type.to_owned(),
            description: self.description.to_owned(),
            discount_card_number: self.discount_card_number.to_owned(),
            email: self.email.to_owned(),
            external_code: self.external_code.to_owned(),
            fax: self.fax.to_owned(),
            name: self.name.to_owned(),
            phone: self.phone.to_owned(),
            price_type: self.price_type.to_owned(),
            shared: self.shared,
            state: self.state.to_owned(),
            tags: self.tags.to_owned(),
            legal_title: self.legal_title.to_owned(),
            legal_address: self.legal_address.to_owned(),
            legal_address_full: self.legal_address_full.to_owned(),
            inn: self.inn.to_owned(),
            kpp: self.kpp.to_owned(),
            ogrn: self.ogrn.to_owned(),
            okpo: self.okpo.to_owned(),
            ogrnip: self.ogrnip.to_owned(),
            legal_last_name: self.legal_last_name.to_owned(),
            legal_first_name: self.legal_first_name.to_owned(),
            legal_middle_name: self.legal_middle_name.to_owned(),
            sex: self.sex.to_owned(),
            certificate_number: self.certificate_number.to_owned(),
            certificate_date: self.certificate_date.to_owned(),
            archived: self.archived,
            code: self.code.to_owned(),
            meta: self.meta.to_owned(),
        }
    }
}
