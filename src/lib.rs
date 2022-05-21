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

    pub fn read_json_config<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
        let file = File::open(path).expect("Cannot read file");
        let reader = BufReader::new(file);

        // read json from file
        let json_data = serde_json::from_reader(reader)?;

        Ok(json_data)
    }
}

pub mod db_controller {
    use chrono::{serde::ts_seconds, DateTime, Utc};
    use mongodb::{bson::doc, sync::Client};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    // Task型
    pub struct Task {
        #[serde(with = "ts_seconds")]
        pub scheduled_date: DateTime<Utc>,
        #[serde(with = "ts_seconds")]
        pub register_date: DateTime<Utc>,
        pub title: String,
        pub memo: String,
        pub finish: bool,
    }

    //新規タスクの追加
    pub fn add_task(new_task: Task, db_address: String) -> Result<(), mongodb::error::Error> {
        let client = Client::with_uri_str(db_address)?;
        let database = client.database("taskdb");
        let collection = database.collection::<Task>("task");

        collection.insert_one(new_task, None)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        config::{read_json_config, Config},
        db_controller::{add_task, Task},
    };
    use chrono::prelude::Utc;

    #[test]
    fn add_new_task() {
        let new_task = Task {
            scheduled_date: Utc::now(),
            register_date: Utc::now(),
            title: "LLP".into(),
            memo: "Life Love Peace".into(),
            finish: false,
        };
        let db_address = String::from("mongodb://localhost:27017");

        add_task(new_task, db_address).expect("Failed to add new Task");
        println!("add new Task");
    }
    #[test]
    fn read_config_from_json() {
        let config: Config =
            read_json_config("test/config.json").expect("Cannot read Json config.");
        assert_eq!(config.db_address, "mongodb://localhost:27017");
    }
}
