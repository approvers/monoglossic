use chrono::{
    serde::{ts_seconds, ts_seconds_option},
    DateTime, Utc,
};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MonoglossicError {
    #[error("DB io Error {0}")]
    Io(String),
}

pub trait MonoglossicRepository {
    fn add_task(&mut self, task: Task) -> Result<(), MonoglossicError>;
}

#[derive(Debug, Serialize, Deserialize)]

// Task型
pub struct Task {
    #[serde(with = "ts_seconds_option")]
    pub scheduled_date: Option<DateTime<Utc>>,
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
            title: String::new(),
            memo: String::new(),
            finish: false,
        }
    }
}

/*
 *
 */
