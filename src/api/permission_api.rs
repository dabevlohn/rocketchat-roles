use crate::{models::permission_model::Permission, repository::mongodb_repo::MongoRepo};
use rocket::{http::Status, serde::json::Json, State};

#[get("/permissions")]
pub fn get_all_permissions(db: &State<MongoRepo>) -> Result<Json<Vec<Permission>>, Status> {
    let permissions = db.get_all_permissions();
    match permissions {
        Ok(permissions) => Ok(Json(permissions)),
        Err(_) => Err(Status::InternalServerError),
    }
}
