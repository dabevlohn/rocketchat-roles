use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::models::permission_model::Permission;
use crate::models::role_model::Role;
use crate::models::user_model::User;
use mongodb::{
    bson::doc,
    bson::extjson::de::Error,
    sync::{Client, Collection},
    //    results::InsertOneResult,
};

pub struct MongoRepo {
    usercol: Collection<User>,
    rolecol: Collection<Role>,
    permcol: Collection<Permission>,
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
        let usercol: Collection<User> = db.collection("users");
        let rolecol: Collection<Role> = db.collection("rocketchat_roles");
        let permcol: Collection<Permission> = db.collection("rocketchat_permissions");
        MongoRepo {
            usercol,
            permcol,
            rolecol,
        }
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
            .usercol
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let cursors = self
            .usercol
            .find(None, None)
            .ok()
            .expect("Error getting list of users");
        let users = cursors.map(|doc| doc.unwrap()).collect();
        Ok(users)
    }

    pub fn get_all_roles(&self) -> Result<Vec<Role>, Error> {
        let cursors = self
            .rolecol
            .find(None, None)
            .ok()
            .expect("Error getting list of roles");
        let roles = cursors.map(|doc| doc.unwrap()).collect();
        Ok(roles)
    }

    pub fn get_all_permissions(&self) -> Result<Vec<Permission>, Error> {
        let filter =
            doc! { "$nor": [ { "roles": { "$exists": false } }, { "roles": { "$size": 0 } } ] };
        let cursors = self
            .permcol
            .find(filter, None)
            .ok()
            .expect("Error getting list of permissions");
        let permissions = cursors.map(|doc| doc.unwrap()).collect();
        Ok(permissions)
    }
}
