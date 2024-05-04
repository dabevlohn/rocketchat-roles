use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::models::user_model::User;
use mongodb::{
    bson::doc,
    bson::extjson::de::Error,
    //    results::InsertOneResult,
    sync::{Client, Collection},
};

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rocketchat");
        let col: Collection<User> = db.collection("users");
        MongoRepo { col }
    }

    /*
    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: "test".to_string(),
            username: new_user.username,
            status: new_user.status,
            active: true,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");
        Ok(user)
    }
    */

    pub fn get_user(&self, id: &String) -> Result<User, Error> {
        // let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let cursors = self
            .col
            .find(None, None)
            .ok()
            .expect("Error getting list of users");
        let users = cursors.map(|doc| doc.unwrap()).collect();
        Ok(users)
    }
}
