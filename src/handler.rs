use std::io::ErrorKind::Other;

use actix_web::{ Responder, web };
use deadpool_postgres::{ Client, Pool };

use crate::models::{ Status, CreateTodoList, Result };
use crate::db;

pub async fn status() -> impl Responder {
	web::HttpResponse::Ok()
		.json(Status { status: "ok".to_string() })
}

pub async fn get_todos(db_pool: web::Data <Pool>) -> impl Responder {
	let client: Client =
		db_pool.get().await.expect("Error connecting to the database!");

	let result = db::get_todos(&client).await;

	match result {
		Ok(todos) => web::HttpResponse::Ok().json(todos),
		Err(_) => web::HttpResponse::InternalServerError().into()
	}
}

pub async fn create_todo(
	db_pool: web::Data <Pool>, json: web::Json <CreateTodoList>
) -> impl Responder {
	let client: Client =
		db_pool.get().await.expect("Error connecting to the database!");

	let result = db::create_todo(&client, json.title.clone()).await;

	match result {
		Ok(todo) => web::HttpResponse::Ok().json(todo),
		Err(_) => web::HttpResponse::InternalServerError().into()
	}
}

pub async fn get_items(
	db_pool: web::Data <Pool>, web::Path ((list_id, )): web::Path <(i32, )>
) -> impl Responder {
	let client: Client =
		db_pool.get().await.expect("Error connecting to the database!");

	let result = db::get_items(&client, list_id).await;

	match result {
		Ok(items) => web::HttpResponse::Ok().json(items),
		Err(_) => web::HttpResponse::InternalServerError().into()
	}
}

pub async fn check_item(
	db_pool: web::Data <Pool>, 
	web::Path ((list_id, item_id)): web::Path <(i32, i32)>
) -> impl Responder {
	let client: Client =
		db_pool.get().await.expect("Error connecting to the database!");

	let result = db::check_item(&client, list_id, item_id).await;

	match result {
		Ok(_todo) => web::HttpResponse::Ok().json(Result { success: true }),
		Err(ref e) if e.kind() == Other => web::HttpResponse::Ok().json(Result { success: false }),
		Err(_) => web::HttpResponse::InternalServerError().into()
	}
}