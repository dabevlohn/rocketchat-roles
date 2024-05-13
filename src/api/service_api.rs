use crate::{models::setting_model::InternalService, repository::localdb_repo::LocalRepo};
use rocket::{http::Status, serde::json::Json, State};

#[get("/services")]
pub fn get_all_services(db: &State<LocalRepo>) -> Result<Json<Vec<InternalService>>, Status> {
    let services = db.get_all_services();
    match services {
        Ok(services) => Ok(Json(services)),
        Err(_) => Err(Status::InternalServerError),
    }
}
