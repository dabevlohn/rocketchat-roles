use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    //#[serde(rename = "_id")]
    //id: String,
    year: i16,
    month: i16,
    day: i16,

    #[serde(rename = "sessionId")]
    session_id: Option<String>,
    #[serde(rename = "userId")]
    user_id: String,
    host: Option<String>,
    #[serde(rename = "searchTerm")]
    search_term: Option<String>,
    #[serde(rename = "mostImportantRole")]
    most_important_role: String,
    #[serde(rename = "createdAt")]
    created_at: Option<DateTime>,
    #[serde(rename = "lastActivityAt")]
    last_activity: Option<DateTime>,
    #[serde(rename = "_updatedAt")]
    updated_at: Option<DateTime>,
    #[serde(rename = "_closedAt")]
    closed_at: Option<DateTime>,
}
