use actix_web::{web,HttpResponse,Responder, delete};
use actix_web::{post,get,patch};
// use sqlx::query;
// use crate::model::task::Task;
use crate::AppState;
use crate::model::task::Task; 
use serde_json::json;
// use serde::{Deserialize,Serialize};


// #[derive(Serialize, Deserialize)]
// pub struct TaskCreateData {
//     task_name: String,
// }

#[post("/tasks")]
async fn create_task(
    // body: web::Json<TaskCreateData>,
    body: web::Json<Task>,
    data: web::Data<AppState>
) -> impl Responder {
    
    let query_result = sqlx::query!(
        "INSERT INTO tasks(task_name) VALUES ($1) RETURNING *",
        body.task_name.to_string()
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


#[get("/tasks/{id}")]
async fn get_task(
    path:web::Path<uuid::Uuid>,
    data:web::Data<AppState>,
) -> impl Responder{

    let task_id = path.into_inner();
    let query_result = sqlx::query!(
        "SELECT * FROM tasks"
        // task_id
    )
    .fetch_one(&data.db)
    .await;

    for query_row in query_result{
        println!("{:?}",query_row);
    }

    // match query_result {
    //     Ok(task) => {
    //         let task_response = serde_json::json!({
    //             "status":"success,
    //             "data": serde_json::json!({"task":task})
    //         });
    //         return HttpResponse::Ok().json(task_response);
    //     }
    //     Err(_) => {
    //         let message = format!("task id {} not found {} ",task_id)
    //         return HttpResponse::NOTFound()
    //         .json(serde_json::json!({"status":fail,"message":message}));
    //     }
    // }
        ""
}




// #[patch("/tasks/{id}")]


// async fn update_task_handler(
//     path: web::Path<uuid::Uuid>,
//     body: web::Json<Task>,
//     data: web::Data<AppState>,
// ) -> impl Responder {
//         let task_id = path.into_inner();
//         let query_result = sqlx::query!(
//         "SELECT * FROM tasks WHERE id = $1",
//         task_id
//         )
//         .fetch_one(&data.db)
//         .await;

//         if query_result.is_err(){
//             let message = format!("task with id : {} not found ",task_id);
//             return  HttpResponse::NotFound().json(serde_json::json!({"status":"fail","message":message}));
//         }

//         let task = query_result.unwrap();
//         let query_result =sqlx::query!(
//             "UPDATE tasks SET task_name = $1 WHERE id = $2 RETURNING * ",
//             body.task_name.to_owned().unwrap_or(task.task_name),
//             task_id
//         )
//         .fetch_one(&data.db)
//         .await;

//         match query_result{
//             Ok(task) => {
//                     let task_response = serde_json::json!({"status":"success",
//                 "data":serde_json::json!({"task":task})});
//                 return HttpResponse::Ok().json(task_response);
//             }
//             Err(err) => {
//                 let message = format!("Error {:?} ",err);
//                 return HttpResponse::InternalServerError().json(serde_json::json!(
//                     { "status":"error",
//                      "message":message
//                     }));
//             }

//         }

//     ""
// }

// #[delete("/tasks/{id}")]

// async fn delete_task_handler(
//     path:web::Path<uuid::Uuid>,
//     data:web::Data<AppState>,
// ) -> impl Responder{

//     let task_id = path.into_inner();
//         let row_affected = sqlx::query_as!(
//         "DELETE FROM tasks WHERE id = $1",task_id)
//         .execute(&data.db)
//         .await
//         .unwrap()
//         .rows_affected();
    
//     if row_affected == 0 {
//         let message = format!("Note with ID: {} not found", task_id);
//         return HttpResponse::NotFound().json(json!({"status": "fail","message": message}));
//     }

//     HttpResponse::NoContent().finish()

// }


pub fn config(conf: &mut web::ServiceConfig){
    let scope = web::scope("/api")
        .service(create_task)
        .service(get_task);
    conf.service(scope);
}