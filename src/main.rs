use dotenv::dotenv;
use actix_web::{ HttpServer, App, web };

use tokio_postgres::NoTls;

use slog::{ Logger, Drain, o, info };
use slog_term;
use slog_async;

use crate::app::AppState;
use crate::handler::*;

mod app;
mod models;
mod config;
mod handler;
mod db;
mod errors;

fn coonfigure_log() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let console_drain = slog_term::FullFormat::new(decorator).build().fuse();
    let console_drain = slog_async::Async::new(console_drain).build().fuse();
    slog::Logger::root(console_drain, o! ("v" => env! ("CARGO_PKG_VERSION")))
}

#[actix_web::main]
async fn main() -> std::io::Result <()> {
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();

    let log  = coonfigure_log();

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
