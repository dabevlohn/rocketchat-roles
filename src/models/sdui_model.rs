use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SomeVal {
    Str(String),
    Vec(Vec<String>),
    Bool(bool),
    U64(u64),
    I32(i32),
    None,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sdui {
    #[serde(rename = "_id")]
    id: String,
    #[serde(rename = "_updatedAt")]
    updated_at: DateTime,
    ts: DateTime,
    #[serde(rename = "i18nLabel")]
    i18n_label: String,
    #[serde(rename = "i18nDescription")]
    i18n_desc: String,
    #[serde(rename = "type")]
    utype: String,
    #[serde(rename = "valueSource")]
    value_source: String,
    #[serde(rename = "packageValue")]
    package_value: Option<SomeVal>,
    value: Option<SomeVal>,
    section: String,
    group: String,
    public: bool,
    secret: bool,
    hidden: bool,
    blocked: bool,
    enterprise: bool,
    env: bool,
    autocomplete: bool,
    sorter: Option<i16>,
}
