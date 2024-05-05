use crate::{models::role_model::Role, repository::mongodb_repo::MongoRepo};
use rocket::{http::Status, serde::json::Json, State};

#[get("/roles")]
pub fn get_all_roles(db: &State<MongoRepo>) -> Result<Json<Vec<Role>>, Status> {
    let roles = db.get_all_roles();
    match roles {
        Ok(roles) => Ok(Json(roles)),
        Err(_) => Err(Status::InternalServerError),
    }
}
