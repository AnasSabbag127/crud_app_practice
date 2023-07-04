use serde::{ Serialize, Deserialize};
use uuid::Uuid;
use strum_macros::{EnumString, Display};


#[derive(Serialize,Deserialize, EnumString, Display, Eq, PartialEq)]
pub enum TaskState {
    NotStarted,
    InProgress,
    Completed,
    Paused,
    Failed
}

#[derive(Serialize, Deserialize)]
pub struct Task{
    pub id: Uuid,
    pub task_name: String,
    pub task_state: TaskState,
    pub user_id: Uuid,
}


