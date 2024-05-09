use crate::{models::sdui_model::Sdui, repository::mongodb_repo::MongoRepo};
use rocket::{http::Status, serde::json::Json, State};

#[get("/layout")]
pub fn get_full_layout(db: &State<MongoRepo>) -> Result<Json<Vec<Sdui>>, Status> {
    let layout = db.get_full_layout();
    match layout {
        Ok(layout) => Ok(Json(layout)),
        Err(_) => Err(Status::InternalServerError),
    }
}
