use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use rocket::{http::Status, serde::json::Json, State};
// use mongodb::bson::Document;
// use mongodb::results::InsertOneResult;
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn index(db: &State<MongoRepo>) -> Template {
    let users = db.get_all_users_obj();
    match users {
        Ok(users) => Template::render("index", context! { users: users }),
        Err(_) => Template::render("error/404", context! { uri: "" }),
    }
}

/*
#[post("/user", data = "<new_user>")]
pub fn create_user(
    db: &State<MongoRepo>,
    new_user: Json<User>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = User {
        id: None,
        username: new_user.username.to_owned(),
        status: new_user.status.to_owned(),
        active: true,
    };
    let user_detail = db.create_user(data);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}
*/

#[get("/user/<path>")]
pub fn get_user(db: &State<MongoRepo>, path: String) -> Result<Json<User>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let user_detail = db.get_user(&id);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/users")]
pub fn get_all_users(db: &State<MongoRepo>) -> Result<Json<Vec<User>>, Status> {
    let users = db.get_all_users_obj();
    match users {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(Status::InternalServerError),
    }
}
