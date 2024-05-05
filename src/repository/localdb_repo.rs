use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::models::setting_model::Setting;
use mongodb::{
    // bson::extjson::de::Error,
    // results::InsertOneResult,
    sync::{Client, Collection},
};

#[allow(dead_code)]
pub struct LocalRepo {
    setcol: Collection<Setting>,
}

impl LocalRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("LOCALURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("mkbadmin");
        let setcol: Collection<Setting> = db.collection("settings");
        LocalRepo { setcol }
    }

    /*
    pub fn create_setting(&self) -> Result<InsertOneResult, Error> {
        let new_doc = Setting {
            id: None,
            username: "admin".to_string(),
            token: "0123456789".to_string(),
            active: true,
        };
        let setting = self
            .setcol
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating setting");
        Ok(setting)
    }

    pub fn get_all_settings(&self) -> Result<Vec<Setting>, Error> {
        // let _ = self.create_setting();
        let cursors = self
            .setcol
            .find(None, None)
            .ok()
            .expect("Error getting list of settings");
        let users = cursors.map(|doc| doc.unwrap()).collect();
        Ok(users)
    }
    */
}
