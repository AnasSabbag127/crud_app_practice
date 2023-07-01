use actix_web::{web,HttpResponse,Responder};
use actix_web::post;
use crate::model::task::Task;
use crate::AppState; 
use serde_json::json;
use serde::{Deserialize,Serialize};


#[derive(Serialize, Deserialize)]
pub struct TaskCreateData {
    task_name: String,
}

#[post("/tasks")]
async fn task_handler(
    body: web::Json<TaskCreateData>,
    data: web::Data<AppState>
) -> impl Responder {
    
    let query_result = sqlx::query!(
        "INSERT INTO tasks(task_name) VALUES ($1) RETURNING *",
        body.task_name
        // body.task_name.to_string().unwrap_or("".to_string())
    )
    .fetch_one(&data.db)
    .await;

    // match query_result {
    //     Ok(task) => {
    //         let task_response = serde_json::json!({"status": "success","data": serde_json::json!({
    //             "task": task
    //         })});

    //         return HttpResponse::Ok().json(task_response);
    //     }
    //     Err(e) => {
    //         if e.to_string()
    //             .contains("duplicate key value violates unique constraint")
    //         {
    //             return HttpResponse::BadRequest()
    //             .json(serde_json::json!({"status": "fail","message": "Note with that title already exists"}));
    //         }

    //         return HttpResponse::InternalServerError()
    //             .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
    //     }
    // }
    ""

}
