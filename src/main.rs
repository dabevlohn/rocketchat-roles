// use mongodb::{
//    options::{ClientOptions, ResolverConfig},
//    Client,
// };
use chrono::Utc;
use futures::stream::TryStreamExt;
use mongodb::{
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
    userId: String,
    loginToken: String,
    roles: Vec<String>,
    mostImportantRole: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    loginAt: chrono::DateTime<Utc>,
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

    let typed_collection = db.collection::<Session>("rocketchat_sessions");

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

    let filter = doc! { "host": "localhost:3000"  };
    let find_options = FindOptions::builder().sort(doc! { "_id": 1 }).build();
    let mut cursor = typed_collection.find(filter, find_options).await?;

    // Iterate over the results of the cursor.
    while let Some(session) = cursor.try_next().await? {
        println!(
            "userId: {}, loginToken: {}, loginAt: {}, role: {}, roles: {:?}",
            session.userId,
            session.loginToken,
            session.loginAt,
            session.mostImportantRole,
            session.roles
        );
    }

    Ok(())
}
