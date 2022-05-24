use crate::monoglossic_repository::{MonoglossicError, MonoglossicRepository, Task};
use mongodb::sync::Collection;

pub struct MonoglossicMongoRepository {
    mongodb_collection: Collection<Task>,
}

impl MonoglossicRepository for MonoglossicMongoRepository {
    //新規タスクの追加
    fn add_task(&mut self, task: Task) -> Result<(), MonoglossicError> {
        self.mongodb_collection
            .insert_one(task, None)
            .map_err(|error| MonoglossicError::Io(format!("{:?}", error)))?;
        Ok(())
    }
}
