use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::models::setting_model::InternalService;
use mongodb::{
    bson::extjson::de::Error,
    // results::InsertOneResult,
    sync::{Client, Collection},
};

pub struct LocalRepo {
    // setcol: Collection<Setting>,
    servcol: Collection<InternalService>,
}

impl LocalRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("LOCALURI") {
            Ok(v) => v.to_string(),
            Err(_) => "Error loading env variable".to_owned(),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("mkbadmin");
        // let setcol: Collection<Setting> = db.collection("settings");
        let servcol: Collection<InternalService> = db.collection("services");
        LocalRepo {
            // setcol,
            servcol,
        }
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

    // pub fn create_service(&self) -> Result<InsertOneResult, Error> {
    //     let new_doc = InternalService {
    //         id: None,
    //         name: "CorpPortal1".to_string(),
    //         uri: "https://mcb.ru/".to_string(),
    //         roles: vec!["user".to_string(), "admin".to_string()],
    //         active: true,
    //     };
    //     let service = self
    //         .servcol
    //         .insert_one(new_doc, None)
    //         .expect("Error creating service");
    //     Ok(service)
    // }

    pub fn get_all_services(&self) -> Result<Vec<InternalService>, Error> {
        // let _ = self.create_service();
        let cursors = self
            .servcol
            .find(None, None)
            .expect("Error getting list of services");
        let services = cursors.map(|doc| doc.unwrap()).collect();
        Ok(services)
    }
}
