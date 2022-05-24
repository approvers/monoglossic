pub mod config;
pub mod mongodb;
pub mod monoglossic_repository;
#[cfg(test)]
#[cfg(FALSE)]
mod tests {
    use crate::config::{read_json_config, Config};
    use chrono::{TimeZone, Utc};
    use mongodb::sync::Client;

    #[test]
    fn add_new_task_schedule_none() {
        let new_task = Task {
            title: "LLP".into(),
            memo: "Life Love Peace".into(),
            ..Default::default()
        };
        let client =
            Client::with_uri_str("mongodb://localhost:27017").expect("Cannot connect to DB");
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
        let client =
            Client::with_uri_str("mongodb://localhost:27017").expect("Cannot connect to DB");
        let database = client.database("testdb");
        let collection = database.collection::<Task>("task");

        add_task(new_task, &collection).expect("Failed to add new Task");
    }

    #[test]
    fn read_config_from_json() {
        let config: Config =
            read_json_config("test/config.json").expect("Cannot read `test/config.json`.");
        assert_eq!(config.db_address, "mongodb://localhost:27017");
    }
}
