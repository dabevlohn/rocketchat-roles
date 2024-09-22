// use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Role struct
#[derive(Debug, ToSchema, Serialize, Deserialize)]
pub struct Role {
    /// Role name is the same as it's ID and is unique
    #[serde(rename = "_id")]
    id: String,
    /// No idea what it is
    scope: String,
    /// Role name is the same as it's ID
    name: String,
}
