use clap::Result;
use mongodb::{bson::doc, sync::Client};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
}
fn main() -> Result<(), mongodb::error::Error> {
    let client = Client::with_uri_str("mongodb://localhost:27017")?;
    let database = client.database("mydb");
    let collection = database.collection::<Book>("books");

    let docs = vec![
        Book {
            title: "1984".to_string(),
            author: "George Orwell".to_string(),
        },
        Book {
            title: "Animal Farm".to_string(),
            author: "F. Soctt Fitzgerald".to_string(),
        },
    ];

    collection.insert_many(docs, None)?;

    let cursor = collection.find(doc! {"author": "George Orwell"}, None)?;

    for result in cursor {
        println!("title: {}", result?.title);
    }
    Ok(())
}
