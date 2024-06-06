use crate::repository::mongodb_repo::MongoRepo;
use rocket::{http::Status, serde::json::Json, State};
use mongodb::bson::Document;

#[get("/rawdocs")]
pub fn get_all_docs(db: &State<MongoRepo>) -> Result<Json<Vec<Document>>, Status> {
    let docs = db.get_all_docs();
    match docs {
        Ok(docs) => Ok(Json(docs)),
        Err(_) => Err(Status::InternalServerError),
    }
}
