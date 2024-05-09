use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::models::permission_model::Permission;
use crate::models::role_model::Role;
use crate::models::room_model::Room;
use crate::models::user_model::User;
use chrono::{TimeZone, Utc};
use mongodb::{
    bson::{doc, Document},
    options::FindOptions,
    bson::extjson::de::Error,
    sync::{Client, Collection},
    //    results::InsertOneResult,
};

pub struct MongoRepo {
    usercol: Collection<User>,
    rolecol: Collection<Role>,
    permcol: Collection<Permission>,
    roomcol: Collection<Room>,
    // roomcolraw: Collection<Document>,
    // usercolraw: Collection<Document>,
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
        let roomcol: Collection<Room> = db.collection("rocketchat_room");
        // let roomcolraw: Collection<Document> = db.collection("rocketchat_room");
        // let usercolraw: Collection<Document> = db.collection("users");
        MongoRepo {
            usercol,
            permcol,
            rolecol,
            roomcol,
            // roomcolraw,
            // usercolraw,
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
    // pub fn get_all_users(&self) -> Result<Vec<Document>, Error> {
        let trashold = Utc.ymd(2024,1,1).and_hms_opt(0,0,0);
        let filter =
            doc! { "$nor": [ 
                { "roles": { "$exists": false } }, 
                { "roles": { "$size": 0 } }, 
                { "roles": { "$in": ["Deactivated"] } }, 
                { "__rooms": { "$exists": false } }, 
                { "__rooms": { "$size": 0 } },
                { "active": false },
                { "lastLogin": { "$exists": false } },
                { "lastLogin": { "$lt": trashold } },
                { "emails.verified": false }
            ] };
        let cursors = self
            .usercol
            .find(filter, None)
            .ok()
            .expect("Error getting list of users");
        let users = cursors.map(|doc| doc.unwrap()).collect();
        Ok(users)
    }

    pub fn get_all_rooms(&self) -> Result<Vec<Room>, Error> {
    // pub fn get_all_rooms(&self) -> Result<Vec<Document>, Error> {
        let trashold = Utc.ymd(2024,4,1).and_hms_opt(0,0,0);
        let filter =
            doc! { "$nor": [ 
                { "usersCount": { "$exists": false } }, 
                { "usersCount": { "$lt": 2 } }, 
                { "msgs": { "$exists": false } }, 
                { "msgs": { "$lt": 1 } }, 
                { "_updatedAt": { "$lt": trashold } }
            ] };
        //let find_options = None;
        let find_options = FindOptions::builder().sort(doc! { "msgs": 1, "usersCount": 1 }).build();
        let cursors = self
            .roomcol
            .find(filter, find_options)
            .ok()
            .expect("Error getting list of rooms");
        let rooms = cursors.map(|doc| doc.unwrap()).collect();
        Ok(rooms)
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
