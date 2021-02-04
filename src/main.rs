use dotenv::dotenv;
use actix_web::{ HttpServer, App, web };

use tokio_postgres::NoTls;

use crate::handler::*;

mod models;
mod config;
mod handler;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result <()> {
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();

    println! (
        "Starting server at http://{}:{}",
        config.server.host, config.server.port
    );

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(status))
            .route("/todos", web::get().to(get_todos))
            .route("/todos", web::post().to(create_todo))
            .route("/todos/{list_id}/items", web::get().to(get_items))
            .route("/todos/{list_id}/items/{item_id}", web::put().to(check_item))
    })
    .bind(format! ("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
