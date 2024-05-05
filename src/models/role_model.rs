// use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    #[serde(rename = "_id")]
    id: String,
    scope: String,
    name: String,
}
