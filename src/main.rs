mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;
use rocket::{get, http::Status, serde::json::Json};

use api::permission_api::get_all_permissions;
use api::role_api::get_all_roles;
use api::user_api::{get_all_users, get_user};
use repository::mongodb_repo::MongoRepo;

#[get("/")]
fn hello() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello from rust and mongoDB")))
}

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![get_user])
        .mount("/", routes![get_all_users])
        .mount("/", routes![get_all_roles])
        .mount("/", routes![get_all_permissions])
        .mount("/", routes![hello])
}

/*
#[derive(Debug, Serialize, Deserialize)]
struct Session {
    host: String,
    roles: Vec<String>,
    #[serde(rename = "userId")]
    user_id: String,
    #[serde(rename = "loginToken")]
    login_token: String,
    #[serde(rename = "mostImportantRole")]
    most_important_role: String,
    #[serde(
        rename = "loginAt",
        with = "bson::serde_helpers::chrono_datetime_as_bson_datetime"
    )]
    login_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Role {
    #[serde(rename = "_id")]
    id: String,
    // #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    // id: Option<ObjectId>,
    scope: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Permission {
    #[serde(rename = "_id")]
    id: String,
    roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    #[serde(rename = "_id")]
    id: String,
    roles: Vec<String>,
    status: String,
    username: String,
    active: bool,
}
*/
