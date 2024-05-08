// use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct Email {
    address: String,
    verified: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    id: String,
    roles: Vec<String>,
    status: String,
    #[serde(rename = "statusDefault")]
    status_default: Option<String>,
    #[serde(rename = "statusConnection")]
    status_connection: Option<String>,
    #[serde(rename = "__rooms")]
    rooms: Option<Vec<String>>,
    username: String,
    name: Option<String>,
    #[serde(rename = "type")]
    utype: Option<String>,
    emails: Option<Vec<Email>>,
    active: bool,
    #[serde(rename = "createdAt")]
    created_at: DateTime,
    #[serde(rename = "lastLogin")]
    last_login: Option<DateTime>,
    #[serde(rename = "_updatedAt")]
    updated_at: Option<DateTime>,
}
