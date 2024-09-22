mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

use crate::models::role_model;
use api::{
    permission_api::get_all_permissions,
    rawdoc_api::get_all_docs,
    role_api,
    room_api::get_all_rooms,
    sdui_api::get_full_layout,
    service_api::get_all_services,
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
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

// for test only
#[get("/hello")]
fn hello() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello world")))
}

#[derive(OpenApi)]
#[openapi(
    paths(
        role_api::get_all_roles,
        role_api::get_role,
    ),
    components(
        schemas(role_model::Role)
    ),
    tags(
        (name = "Role", description = "Role management endpoints.")
    ),
    // modifiers(&SecurityAddon)
)]
struct ApiDoc;

#[launch]
fn rocket() -> _ {
    let ldb = LocalRepo::init();
    let db = MongoRepo::init();
    rocket::build()
        .manage(ldb)
        .manage(db)
        .mount("/", FileServer::from(relative!("static")))
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .mount("/", RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .mount("/", Redoc::with_url("/redoc", ApiDoc::openapi()))
        .mount(
            "/",
            routes![
                get_user,
                get_all_services,
                get_user_email,
                get_user_status,
                role_api::get_role,
                get_all_users,
                get_users_by_role,
                role_api::get_all_roles,
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
