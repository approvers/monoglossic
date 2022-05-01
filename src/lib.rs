pub mod db_controller {
    use chrono::{serde::ts_seconds, DateTime, Utc};
    use mongodb::{bson::doc, sync::Client};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Task {
        #[serde(with = "ts_seconds")]
        scheduled_date: DateTime<Utc>,
        #[serde(with = "ts_seconds")]
        register_date: DateTime<Utc>,
        title: String,
        memo: String,
        finish: bool,
    }

    pub fn add_task(data: Task) -> Result<(), mongodb::error::Error> {
        let client = Client::with_uri_str("mongodb://localhost27017")?;
        let database = client.database("taskdb");
        let collection = database.collection::<Task>("task");

        let docs = vec![data];
        collection.insert_many(docs, None)?;
        Ok(())
    }
}
