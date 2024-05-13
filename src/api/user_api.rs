use crate::{
    models::user_model::User,
    repository::{localdb_repo::LocalRepo, mongodb_repo::MongoRepo},
};
use rocket::{http::Status, serde::json::Json, State};
// use mongodb::bson::Document;
// use mongodb::results::InsertOneResult;
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn index(db: &State<MongoRepo>, ldb: &State<LocalRepo>) -> Template {
    let users = db.get_all_users_obj();
    match users {
        Ok(users) => {
            let services = ldb.get_all_services();
            match services {
                Ok(services) => {
                    Template::render("index", context! { users: users, services: services })
                }
                Err(_) => Template::render("404", context! { uri: "" }),
            }
        }
        Err(_) => Template::render("404", context! { uri: "" }),
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
pub fn get_user(db: &State<MongoRepo>, path: &str) -> Template {
    let id = path;
    if id.is_empty() {
        return Template::render("404", context! { uri: "" });
    };
    let user_detail = db.get_user(id);
    match user_detail {
        Ok(user_detail) => {
            let sessions = db.get_sessions(id);
            match sessions {
                Ok(sessions) => Template::render(
                    "user_detail",
                    context! { user: user_detail, sessions: sessions },
                ),
                Err(_) => Template::render("404", context! { uri: "" }),
            }
        }
        Err(_) => Template::render("404", context! { uri: "" }),
    }
}

#[get("/user/<path>/status")]
pub fn get_user_status(db: &State<MongoRepo>, path: &str) -> Result<String, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let user = db.get_user(id);
    match user {
        Ok(user) => Ok(user.status.to_string()),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/user/<path>/email")]
pub fn get_user_email(db: &State<MongoRepo>, path: &str) -> Result<String, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let user = db.get_user(id);
    match user {
        Ok(user) => Ok(if let Some(email) = user.emails.unwrap().first() {
            email.address.to_string()
        } else {
            "None".to_string()
        }),
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

#[get("/users/role/<path>")]
pub fn get_users_by_role(db: &State<MongoRepo>, ldb: &State<LocalRepo>, path: &str) -> Template {
    let id = path;
    if id.is_empty() {
        return Template::render("404", context! { uri: "" });
    };
    let users = db.get_users_by_role(id);
    match users {
        Ok(users) => {
            let services = ldb.get_service_by_role(id);
            match services {
                Ok(services) => {
                    Template::render("index", context! { users: users, services: services })
                }
                Err(_) => Template::render("404", context! { uri: "" }),
            }
        }
        Err(_) => Template::render("404", context! { uri: "" }),
    }
}
