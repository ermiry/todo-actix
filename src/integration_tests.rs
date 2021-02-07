use dotenv::dotenv;
use lazy_static::lazy_static;

use slog::info;
use serde_json::json;

use actix_web::{test, web, App};

use tokio_postgres::NoTls;

use crate::app::AppState;
use crate::config::Config;
use crate::handler;
use crate::models;

lazy_static! {
    static ref APP_STATE: AppState = {
        dotenv().ok();

        let log = Config::configure_log();

        info!(log, "Creating static AppState");

        let config = Config::from_env().unwrap();

        let pool = config.pg.create_pool(NoTls).unwrap();

        AppState {
            db_pool: pool.clone(),
            log: log.clone(),
        }
    };
}

#[actix_rt::test]
async fn test_get_todos() {
    let app = App::new()
        .data(APP_STATE.clone())
        .route("/todos{_:/?}", web::get().to(handler::get_todos));

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get().uri("/todos/").to_request();

    let response = test::call_service(&mut app, req).await;

    assert_eq!(response.status(), 200, "GET /todos should return 200");

    let body = test::read_body(response).await;

    let try_todos: Result<Vec<models::TodoList>, serde_json::error::Error> =
        serde_json::from_slice(&body);

    assert!(try_todos.is_ok(), "Response couldn't not be parsed");
}

#[actix_rt::test]
async fn test_create_todos() {
    let app = App::new()
        .data(APP_STATE.clone())
        .route("/todos{_:/?}", web::get().to(handler::get_todos))
        .route("/todos{_:/?}", web::post().to(handler::create_todo));

    let mut app = test::init_service(app).await;

    let todo_title = "Create todo List";

    let create_todo_list = json!({ "title": todo_title });

    let req = test::TestRequest::post()
        .uri("/todos/")
        .header("Content-Type", "application/json")
        .set_payload(create_todo_list.to_string())
        .to_request();

    let response = test::call_service(&mut app, req).await;

    assert_eq!(response.status(), 200, "Status should be 200.");

    let body = test::read_body(response).await;

    let try_created: Result<models::TodoList, serde_json::error::Error> =
        serde_json::from_slice(&body);

    assert!(try_created.is_ok(), "Response couldn't not be parsed");

    let created_list = try_created.unwrap();

    let req = test::TestRequest::get().uri("/todos/").to_request();

    let todos: Vec<models::TodoList> = test::read_response_json(&mut app, req).await;

    let maybe_list = todos.iter().find(|todo| todo.id == created_list.id);

    assert!(maybe_list.is_some(), "Item not found!");
}