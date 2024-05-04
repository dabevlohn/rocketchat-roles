// use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    id: String,
    roles: Vec<String>,
    status: String,
    username: String,
    active: bool,
}
