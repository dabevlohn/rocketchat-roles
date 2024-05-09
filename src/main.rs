mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

use api::{
    permission_api::get_all_permissions,
    sdui_api::get_full_layout,
    role_api::get_all_roles,
    rawdoc_api::get_all_docs,
    room_api::get_all_rooms,
    user_api::{get_all_users, get_user},
};
use repository::{localdb_repo::LocalRepo, mongodb_repo::MongoRepo};
use rocket::{get, http::Status, serde::json::Json};

#[get("/")]
fn hello() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello world")))
}

#[launch]
fn rocket() -> _ {
    let _ldb = LocalRepo::init();
    let mdb = MongoRepo::init();
    rocket::build()
        .manage(mdb)
        .mount("/", routes![get_user])
        .mount("/", routes![get_all_users])
        .mount("/", routes![get_all_roles])
        .mount("/", routes![get_all_rooms])
        .mount("/", routes![get_full_layout])
        .mount("/", routes![get_all_docs])
        .mount("/", routes![get_all_permissions])
        .mount("/", routes![hello])
}
