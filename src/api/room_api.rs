use crate::{models::room_model::Room, repository::mongodb_repo::MongoRepo};
// use mongodb::bson::Document;
use rocket::{http::Status, serde::json::Json, State};

#[get("/rooms")]
pub fn get_all_rooms(db: &State<MongoRepo>) -> Result<Json<Vec<Room>>, Status> {
// pub fn get_all_rooms(db: &State<MongoRepo>) -> Result<Json<Vec<Document>>, Status> {
    let rooms = db.get_all_rooms();
    match rooms {
        Ok(rooms) => Ok(Json(rooms)),
        Err(_) => Err(Status::InternalServerError),
    }
}
