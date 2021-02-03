use dotenv::dotenv;
use actix_web::{ HttpServer, App, Responder, web };

use crate::models::Status;

mod models;
mod config;

async fn status() -> impl Responder {
   web::HttpResponse::Ok()
    .json(Status { status: "ok".to_string() })
}

#[actix_web::main]
async fn main() -> std::io::Result <()> {
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();

    println! (
        "Starting server at http://{}:{}",
        config.server.host, config.server.port
    );

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(status))
    })
    .bind(format! ("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
