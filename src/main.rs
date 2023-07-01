mod model;
mod repository;
mod api;

use actix_web::{web,App,HttpServer,Responder,HttpResponse};
// use actix_web::{http::header};
// use actix_web::middleware::Logger;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions,Pool,Postgres};



// use api::task::{
//     get_task,
//     submit_task,
//     start_task,
//     complete_task,
//     fail_task,
// }


pub struct AppState{
    db:Pool<Postgres>
}


async fn health_check()->impl Responder{
    HttpResponse::Ok()
}



#[actix_web :: main]

async fn main() -> std::io::Result<()> {
    
    if std::env::var_os("RUST_LOG").is_none(){
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL")
                .expect("database url must be set");
    let pool =  match PgPoolOptions::new()
                .max_connections(10)
                .connect(&database_url)
                .await
                {
                    Ok(pool) => {
                        println!("connection to database is successful ! ");
                        pool
                    }
                    Err(err) => {
                        println!(" failed to  connect to database {:?} ",err);
                        std::process::exit(1);
                    }
                };

            println!("server connected successfully ");

    HttpServer:: new(move ||{
        App::new()
        .app_data(web::Data::new(AppState{db:pool.clone()}))
        .route("/health_check",web::get().to(health_check))

    })
    .bind("127.0.0.1:8000")?
    .run()
    .await

}
