use aws_sdk_dynamodb:: Client;
use aws_sdk_dynamodb::model:: AttributeValue;
use aws_config:: Config;

use crate::model::{Task,TaskState};


use std::str::FromStr;
use std::collections::HashMap;


pub struct DDBRepository{
    client: Client,
    table_name: String
}

pub struct DDBError;

impl DDBRepository {
    
    pub fn init(table_name: String,config: Config) -> DDBRepository{
        let client = Client:: new(&Config);
        DDBRepository { 
            table_name,
            client
        }
    }

    pub async fn put_task(&self, task: Task) -> Result<(), DDBError>{

        let mut request = self.client.put_item()
                .table_name(self.table_name);
                //add item (HashMap) key : value in task all field

        match request.send().await {
            Ok(_) => Ok(()),
            Err(_) => Err(DDBError)
        }

    
    }



    pub async fn get_task(&self,task_id: String) -> Option<Task>{

        let tokens:Vec<String> = task_id
                .split("_")
                .map(|x|String::from(x))
                .collect();

        let user_uuid = AttributeValue::S(tokens[0].clone());
        let task_uuid = AttributeValue::S(tokens[1].clone());

        let res = self.client
            .query()
            .table_name(&self.table_name)
            /*
                add  key_condition_expression
                expression_attribute_names
            */
            .send()
            .await;

        //to read.. ++ 
        return match res {
            Ok(output) => {
                match output.items {
                    Some(items) =>{
                        let item = & items.first()?;
                        error!("{:?}",&item);
                    }
                    match item_to_task(item){
                        Ok(task) => Some (task),
                        Err(_) => None
                    }
                },
                Err(error) => {
                    error!("{:?}",error);
                    None
                }
            }
        }

    }


}
