pub mod config {
    use serde::Deserialize;

    use std::error::Error;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;

    #[derive(Deserialize, Debug)]
    pub struct Config {
        pub db_address: String,
    }

    pub fn read_json_config(path: impl AsRef<Path>) -> Result<Config, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        // read json from file
        let json_data = serde_json::from_reader(reader)?;

        Ok(json_data)
    }
}

pub mod db {
    use chrono::{serde::ts_seconds, serde::ts_seconds_option, DateTime, Utc};
    use mongodb::{bson::doc, sync::Collection};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    // Task型
    pub struct Task {
        #[serde(with = "ts_seconds_option")]
        pub scheduled_date: Option<chrono::DateTime<Utc>>,
        #[serde(with = "ts_seconds")]
        pub register_date: DateTime<Utc>,
        pub title: String,
        pub memo: String,
        pub finish: bool,
    }

    impl Default for Task {
        fn default() -> Self {
            Self {
                scheduled_date: None,
                register_date: Utc::now(),
                title: "".into(),
                memo: "".into(),
                finish: false,
            }
        }
    }

    //新規タスクの追加
    pub fn add_task(
        new_task: Task,
        collection: &Collection<Task>,
    ) -> Result<(), mongodb::error::Error> {
        collection.insert_one(new_task, None)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        config::{read_json_config, Config},
        db::{add_task, Task},
    };
    use chrono::{TimeZone, Utc};
    use mongodb::sync::Client;

    #[test]
    fn add_new_task_schedule_none() {
        let new_task = Task {
            title: "LLP".into(),
            memo: "Life Love Peace".into(),
            ..Default::default()
        };
        let client = Client::with_uri_str("mongodb://localhost:27017").expect("Cannot connect to DB");
        let database = client.database("testdb");
        let collection = database.collection::<Task>("task");

        add_task(new_task, &collection).expect("Failed to add new Task");
    }

    #[test]
    fn add_new_task_scheduled() {
        let new_task = Task {
            scheduled_date: Some(Utc.ymd(2022, 1, 23).and_hms(0, 0, 0)),
            title: "LLP".into(),
            memo: "Life Love Peace".into(),
            ..Default::default()
        };
        let client = Client::with_uri_str("mongodb://localhost:27017").expect("Cannot connect to DB");
        let database = client.database("testdb");
        let collection = database.collection::<Task>("task");

        add_task(new_task, &collection).expect("Failed to add new Task");
    }

    #[test]
    fn read_config_from_json() {
        let config: Config =
            read_json_config("test/config.json").expect("Cannot read Json config.");
        assert_eq!(config.db_address, "mongodb://localhost:27017");
    }
}
