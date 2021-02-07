use actix_web::{ Responder, web, HttpResponse };
use deadpool_postgres::{ Client, Pool, PoolError };
use slog::{crit, error, o, Logger};

use crate::app::AppState;
use crate::models::{ Status, CreateTodoList, CreateTodoItem, ResultResponse };
use crate::errors::{ AppError };
use crate::db;

async fn get_client(
	pool: Pool, log: Logger
) -> Result<Client, AppError> {

    pool.get().await.map_err(|err: PoolError| {
        let sublog = log.new(o!("cause" => err.to_string()));
        crit!(sublog, "Error creating client");
		AppError::db_error(err)
		
    })
}

fn log_error(log: Logger) -> Box <dyn Fn(AppError) -> AppError> {

    Box::new(move |err| {
        let log = log.new(o!(
                "cause" => err.cause.clone()
            ));
        error!(log, "{}", err.message());
        err
	})
	
}

pub async fn status() -> impl Responder {
	web::HttpResponse::Ok()
		.json(Status { status: "ok".to_string() })
}

pub async fn get_todos(
	app_state: web::Data <AppState>
) -> Result <impl Responder, AppError> {

	let sublog = app_state.log.new(o!("handler" => "create_todo"));

    let client: Client = get_client(
		app_state.db_pool.clone(), sublog.clone()
	).await?;

    let result = db::get_todos(&client).await;

    result.map(|todos| HttpResponse::Ok().json(todos))
        .map_err(log_error(sublog))

}

pub async fn create_todo(
	app_state: web::Data <AppState>, todo_list: web::Json <CreateTodoList>
) -> Result <impl Responder, AppError> {
	
	let sublog = app_state.log.new(
		o!(
			"handler" => "create_todo",
			"todo_list" => todo_list.title.clone()
		)
	);

    let client: Client = get_client(
		app_state.db_pool.clone(), sublog.clone()
	).await?;

    let result = db::create_todo(&client, todo_list.title.clone()).await;

    result.map(|todo| HttpResponse::Ok().json(todo))
        .map_err(log_error(sublog))

}

pub async fn get_todo(
	app_state: web::Data <AppState>, web::Path ((list_id, )): web::Path <(i32, )>
) -> Result <impl Responder, AppError> {

    let sublog = app_state.log.new(
		o!(
			"handler" => "get_todo",
			"list_id" => list_id
		)
	);

    let client: Client = get_client(
		app_state.db_pool.clone(), sublog.clone()
	).await?;

    let result = db::get_todo(&client, list_id).await;

    result.map(|todo| HttpResponse::Ok().json(todo))
        .map_err(log_error(sublog))

}

pub async fn get_items(
	app_state: web::Data <AppState>, web::Path ((list_id, )): web::Path <(i32, )>
) -> Result <impl Responder, AppError> {

	let sublog = app_state.log.new(
		o!(
			"handler" => "items",
			"list_id" => list_id
		)
	);

	let client: Client = get_client(
		app_state.db_pool.clone(), sublog.clone()
	).await?;

    let result = db::get_items(&client, list_id).await;

    result.map(|items| HttpResponse::Ok().json(items))
        .map_err(log_error(sublog))

}

pub async fn create_item(
	app_state: web::Data <AppState>,
	web::Path ((list_id, )): web::Path <(i32, )>,
	todo_item: web::Json <CreateTodoItem>
) -> Result <impl Responder, AppError> {

    let sublog = app_state.log.new(o!(
        "handler" => "create_item",
        "list_id" => list_id,
        "todo_item" => todo_item.title.clone()
    ));

    let client: Client = get_client(
		app_state.db_pool.clone(), sublog.clone()
	).await?;

    let result = db::create_item(
		&client, list_id, todo_item.title.clone()
	).await;

    result.map(|item| HttpResponse::Ok().json(item))
        .map_err(log_error(sublog))
}

pub async fn get_item(
	app_state: web::Data <AppState>, 
	web::Path ((list_id, item_id)): web::Path <(i32, i32)>
) -> Result<impl Responder, AppError> {

    let sublog = app_state.log.new(o!(
        "handler" => "get_item",
        "list_id" => list_id,
        "item_id" => item_id,
    ));

    let client: Client = get_client(
		app_state.db_pool.clone(), sublog.clone()
	).await?;

    let result = db::get_item(&client, list_id, item_id).await;

    result.map(|item| HttpResponse::Ok().json(item))
        .map_err(log_error(sublog))

}

pub async fn check_item(
	app_state: web::Data <AppState>, 
	web::Path ((list_id, item_id)): web::Path <(i32, i32)>
) -> Result <impl Responder, AppError> {

	let sublog = app_state.log.new(
		o!(
			"handler" => "check_todo",
			"list_id" => list_id,
			"item_id" => item_id,
		)
	);

	let client: Client = get_client(
		app_state.db_pool.clone(), sublog.clone()
	).await?;

    let result = db::check_item(&client, list_id, item_id).await;

    result.map(|updated| HttpResponse::Ok().json(ResultResponse{ success: updated }))
        .map_err(log_error(sublog))

}