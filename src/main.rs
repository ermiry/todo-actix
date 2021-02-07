use dotenv::dotenv;

use actix_web::{ middleware, web, App, HttpServer };

use slog::info;
use tokio_postgres::NoTls;

use crate::app::AppState;
use crate::config::Config;
use crate::handler::*;

mod app;
mod models;
mod config;
mod handler;
mod db;
mod errors;

#[actix_web::main]
async fn main() -> std::io::Result <()> {
    dotenv().ok();

    let config = Config::from_env().unwrap();

    let log  = Config::configure_log();

    let pool = config.pg.create_pool(NoTls).unwrap();

    info! (
        log,
        "Starting server at http://{}:{}",
        config.server.host, config.server.port
    );

    HttpServer::new(move || {
        App::new()
            .data(
                AppState {
                    db_pool: pool.clone(),
                    log: log.clone()
                }
            )
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(status))
            .route("/todos", web::get().to(get_todos))
            .route("/todos", web::post().to(create_todo))
            .route("/todos/{list_id}", web::get().to(get_todo))
            .route("/todos/{list_id}/items", web::get().to(get_items))
            .route("/todos/{list_id}/items", web::post().to(create_item))
            .route("/todos/{list_id}/items/{item_id}", web::get().to(get_item))
            .route("/todos/{list_id}/items/{item_id}", web::put().to(check_item))
    })
    .bind(format! ("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}

#[cfg(test)]
#[cfg(feature = "integration")]
mod integration_tests;
