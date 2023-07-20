use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, From};
use serde_json::json;
use sqlx::postgres::PgDatabaseError;

#[derive(Display, From, Debug)]
pub struct ServiceError(pub(crate) String);

#[derive(Display, From, Debug)]
pub enum CustomError {
    NotFound,
    PGError(PgDatabaseError),
    ServiceError(ServiceError),
}
impl std::error::Error for CustomError {}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        println!("error: {}", &self);
        match *self {
            CustomError::NotFound => HttpResponse::NotFound().finish(),
            CustomError::PGError(ref err) => HttpResponse::InternalServerError()
                .json(json!({"error":-1,"message": err.to_string()})),
            CustomError::ServiceError(ref err) => {
                HttpResponse::Forbidden().json(json!({"error":-1,"message": err.to_string()}))
            }
        }
    }
}
