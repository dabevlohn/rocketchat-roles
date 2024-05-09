// use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct CustomFields {
    #[serde(rename = "Title")]
    title: Option<String>,
    #[serde(rename = "Department")]
    department: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Totp {
    enabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Ldap {
    id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Service {
    ldap: Option<Ldap>,
    totp: Option<Totp>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Email {
    address: String,
    verified: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    id: String,
    username: String,
    name: Option<String>,
    roles: Vec<String>,
    services: Option<Service>,
    status: String,
    emails: Option<Vec<Email>>,
    active: bool,
    ldap: Option<bool>,
    #[serde(rename = "importIds")]
    import_ids: Option<Vec<String>>,
    #[serde(rename = "customFields")]
    custom_fields: Option<CustomFields>,
    #[serde(rename = "statusDefault")]
    status_default: Option<String>,
    #[serde(rename = "statusConnection")]
    status_connection: Option<String>,
    #[serde(rename = "__rooms")]
    rooms: Option<Vec<String>>,
    #[serde(rename = "type")]
    utype: Option<String>,
    #[serde(rename = "createdAt")]
    created_at: DateTime,
    #[serde(rename = "lastLogin")]
    last_login: Option<DateTime>,
    #[serde(rename = "_updatedAt")]
    updated_at: Option<DateTime>,
}
