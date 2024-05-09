// use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct Lmessage {
    msg: Option<String>,
    ts: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    #[serde(rename = "_id")]
    id: String,
    #[serde(rename = "usersCount")]
    users_count: Option<i32>,
    msgs: Option<i32>,
    usernames: Option<Vec<String>>,
    #[serde(rename = "_USERNAMES")]
    user_names: Option<Vec<String>>,
    uids: Option<Vec<String>>,
    name: Option<String>,
    fname: Option<String>,
    topic: Option<String>,
    encrypted: Option<bool>,
    #[serde(rename = "teamMain")]
    team_main: Option<bool>,
    #[serde(rename = "lastMessage")]
    last_message: Option<Lmessage>,
}
