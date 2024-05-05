// use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Email {
    address: String,
    verified: bool,
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
    name: String,
    #[serde(rename = "type")]
    utype: String,
    emails: Option<Vec<Email>>,
    active: bool,
}
