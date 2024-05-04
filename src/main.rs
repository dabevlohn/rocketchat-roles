// use mongodb::{
//    options::{ClientOptions, ResolverConfig},
//    Client,
// };
use chrono::Utc;
use futures::stream::TryStreamExt;
use mongodb::{
    // bson::{oid::ObjectId, Bson},
    bson::doc,
    options::{ClientOptions, FindOptions},
    Client,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use tokio;

#[derive(Debug, Serialize, Deserialize)]
struct Session {
    host: String,
    roles: Vec<String>,
    #[serde(rename = "userId")]
    user_id: String,
    #[serde(rename = "loginToken")]
    login_token: String,
    #[serde(rename = "mostImportantRole")]
    most_important_role: String,
    #[serde(
        rename = "loginAt",
        with = "bson::serde_helpers::chrono_datetime_as_bson_datetime"
    )]
    login_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Role {
    #[serde(rename = "_id")]
    id: String,
    // #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    // id: Option<ObjectId>,
    scope: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Permission {
    #[serde(rename = "_id")]
    id: String,
    roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    #[serde(rename = "_id")]
    id: String,
    roles: Vec<String>,
    status: String,
    username: String,
    active: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load the MongoDB connection string from an environment variable:
    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    // let resolver_config = ResolverConfig::quad9();
    // let options = ClientOptions::parse_with_resolver_config(&client_uri, resolver_config).await?;
    let options = ClientOptions::parse(&client_uri).await?;
    let client = Client::with_options(options)?;

    // Print the databases in our MongoDB cluster:
    // println!("Databases:");
    // for name in client.list_database_names(None, None).await? {
    //    println!("- {}", name);
    // }

    let db = client.database("rocketchat");

    // List the names of the collections in that database.
    // for collection_name in db.list_collection_names(None).await? {
    //    println!("{}", collection_name);
    // }

    println!("---- Sessions ----");
    let session_collection = db.collection::<Session>("rocketchat_sessions");

    /*
    let sessions = vec![
        Session {
            host: "localhost:3000".to_string(),
            userId: "G7RXKXa3w9pMn3RMt".to_string(),
        },
        Session {
            host: "localhost:3000".to_string(),
            userId: "G7RXKXa3w9pMn3RMt".to_string(),
        },
    ];

    typed_collection.insert_many(sessions, None).await?;
    */

    let mut filter = doc! { "host": "localhost:3000"  };
    let mut find_options = FindOptions::builder().sort(doc! { "_id": 1 }).build();
    let mut cursor = session_collection.find(filter, find_options).await?;

    // Iterate over the results of the cursor.
    while let Some(session) = cursor.try_next().await? {
        println!(
            "userId: {}, loginToken: {}, loginAt: {}, role: {}, roles: {:?}",
            session.user_id,
            session.login_token,
            session.login_at,
            session.most_important_role,
            session.roles
        );
    }

    println!("---- Users ----");
    let user_collection = db.collection::<User>("users");
    filter = doc! { "active": true };
    find_options = FindOptions::builder()
        .sort(doc! { "_updatedAt": 1 })
        .build();
    let mut cursor = user_collection.find(filter, find_options).await?;

    while let Some(user) = cursor.try_next().await? {
        println!(
            "id: {}, name: {}, roles: {:?}",
            user.id, user.username, user.roles
        );
    }

    println!("---- Roles ----");
    let role_collection = db.collection::<Role>("rocketchat_roles");
    filter = doc! { "protected": true };
    find_options = FindOptions::builder()
        .sort(doc! { "_updatedAt": 1 })
        .build();
    let mut cursor = role_collection.find(filter, find_options).await?;

    while let Some(role) = cursor.try_next().await? {
        println!("name: {}, scope: {}", role.name, role.scope);
    }

    println!("---- Permissions ----");
    let permission_collection = db.collection::<Permission>("rocketchat_permissions");
    // filter = doc! { "roles": { "$exists": true } };
    filter = doc! { "$nor": [ { "roles": { "$exists": false } }, { "roles": { "$size": 0 } } ] };
    find_options = FindOptions::builder()
        .sort(doc! { "_updatedAt": 1 })
        .build();
    let mut cursor = permission_collection.find(filter, find_options).await?;
    // let mut cursor = permission_collection.find(None, find_options).await?;

    while let Some(permission) = cursor.try_next().await? {
        println!("id: {}, roles: {:?}", permission.id, permission.roles);
    }

    Ok(())
}
