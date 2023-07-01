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
//    pub source_file:String,
//    pub result_file:Option<String>
}


// impl Task{

//     // fn new(user_uuid: String, task_type: String, source_file: String) -> Task{
//     //     Task{
//     //         user_uuid,
//     //         id: Uuid,
//     //         task_type,
//     //         state: TaskState:: NotStarted,
//     //         // source_file,
//     //         // result_file: None
        
//     //     }
//     // }

//     // fn get_global_id(&self) -> String{
//     //     return format!("{}_{}",self.user_uuid,self.task_uuid);
//     // }
//     fn can_transition_to(&self,state: &TaskState) -> bool{
//         self.state != *state
//     }

// }