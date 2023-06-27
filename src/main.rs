mod model;
mod repository;
mod api;

use actix_web :: {web ,App ,HttpRequest ,HttpServer ,Responder, HttpResponse };




// use api::task::{
//     get_task,
//     submit_task,
//     start_task,
//     complete_task,
//     fail_task,
// }


async fn greet(req:HttpRequest) -> impl Responder{
    let meet = req.match_info().get("meet").unwrap_or(" server");
    format!("hello {}! ",meet)
}

async fn health_check()->impl Responder{
    HttpResponse::Ok()
}

#[actix_web :: main]

async fn main() -> std::io::Result<()> {
    // println!("Hello, world!");

    HttpServer:: new(||{
        App::new()
        .route("/",web::get().to(greet))
        .route("/health_check",web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await

}
