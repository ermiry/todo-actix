use std::fmt;

use serde::Serialize;
use actix_web::{ error::ResponseError, http::StatusCode, HttpResponse };
use deadpool_postgres::PoolError;
use tokio_postgres::error::Error;

#[derive(Debug)]
pub enum AppErrorType {
	DBError,
	NotFoundError
}

#[derive(Debug)]
pub struct AppError {
	pub message: Option <String>,
	pub cause: Option <String>,
	pub error_type: AppErrorType
}

impl AppError {
	pub fn message(&self) -> String {
		match &*self {
			AppError { message: Some(message), cause: _, error_type: _ } => message.clone(),
			AppError { message: None, cause: _, error_type: AppErrorType::NotFoundError } => 
				"The requested item was not found".to_string(),

			_ => "An unexpected error has ocurred".to_string()
		}
	}

	pub fn db_error(error: impl ToString) -> AppError {
		AppError { 
			message: None,
			cause: Some(error.to_string()),
			error_type: AppErrorType::DBError
		}
	}
}

impl From <PoolError> for AppError {
    fn from(error: PoolError) -> AppError {
        AppError {
            message: None, 
            cause: Some(error.to_string()),
            error_type: AppErrorType::DBError
        }
    }
}

impl From <Error> for AppError {
    fn from(error: Error) -> AppError {
        AppError {
            message: None, 
            cause: Some(error.to_string()),
            error_type: AppErrorType::DBError
        }
    }
}

impl fmt::Display for AppError {
	fn fmt(&self, f: &mut fmt::Formatter <'_>) -> Result <(), fmt::Error> {
		write! (f, "{:?}", self)
	}
}

impl ResponseError for AppError {
	fn status_code(&self) -> StatusCode {
		match self.error_type {
			AppErrorType::DBError => StatusCode::INTERNAL_SERVER_ERROR,
			AppErrorType::NotFoundError => StatusCode::NOT_FOUND

		}
	}

	fn error_response(&self) -> HttpResponse {
		HttpResponse::build(self.status_code())
			.json(AppErrorResponse { error: self.message() })
	}
}

#[derive(Serialize)]
pub struct AppErrorResponse {
	pub error: String
}