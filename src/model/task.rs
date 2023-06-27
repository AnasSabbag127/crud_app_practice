use serde:: Serialize;
use uuid:: Uuid;
use strum_macros:: { EnumString, Display};


#[derive(Serialize, EnumString, Display, Eq, PartialEq)]

pub enum TaskState {
    NotStartded,
    InProgress,
    Completed,
    Paused,
    Failed
}

#[derive(Serialize)]

pub struct Task{
   pub user_uuid:String,
   pub task_uuid:String,
   pub task_type:String,
   pub state:TaskState,
   pub source_file:String,
   pub result_file:Option<String>
}


impl Task{

    fn new(user_uuid: String, task_type: String, source_file: String) -> Task{
        Task{
            user_uuid,
            task_uuid: Uuid::new_v4().to_string(),
            task_type,
            state: TaskState:: NotStartded,
            source_file,
            result_file: None
        
        }
    }

    fn get_global_id(&self) -> String{
        return format!("{}_{}",self.user_uuid,self.task_uuid);
    }
    fn can_transition_to(&self,state: &TaskState) -> bool{
        self.state != *state
    }

}