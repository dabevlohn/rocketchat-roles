// use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Permission {
    #[serde(rename = "_id")]
    id: String,
    roles: Vec<String>,
}
