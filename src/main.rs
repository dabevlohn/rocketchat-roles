mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

use api::{
    permission_api::get_all_permissions,
    rawdoc_api::get_all_docs,
    role_api::{get_all_roles, get_role},
    room_api::get_all_rooms,
    sdui_api::get_full_layout,
    user_api::{
        get_all_users, get_user, get_user_email, get_user_status, get_users_by_role, index,
    },
};
use repository::{localdb_repo::LocalRepo, mongodb_repo::MongoRepo};
use rocket::{
    fs::{relative, FileServer},
    get,
    http::Status,
    serde::json::Json,
};
use rocket_dyn_templates::Template;

// for test only
#[get("/hello")]
fn hello() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello world")))
}

#[launch]
fn rocket() -> _ {
    let _ldb = LocalRepo::init();
    let mdb = MongoRepo::init();
    rocket::build()
        .manage(mdb)
        .mount("/", FileServer::from(relative!("static")))
        .mount(
            "/",
            routes![
                get_user,
                get_user_email,
                get_user_status,
                get_role,
                get_all_users,
                get_users_by_role,
                get_all_roles,
                get_all_rooms,
                get_full_layout,
                get_all_docs,
                get_all_permissions,
                hello,
                index
            ],
        )
        .attach(Template::fairing())
}
