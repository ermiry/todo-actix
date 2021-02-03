use actix_web::{ Responder, web };
use deadpool_postgres::{ Client, Pool };

use crate::models::Status;
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