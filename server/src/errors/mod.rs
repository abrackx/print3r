use actix_web::{HttpResponse, ResponseError};
use actix_web::http::StatusCode;
use sea_orm::DbErr;
use sqlx::Error as SqlxError;
use sqlx::migrate::MigrateError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Sqlx error: {0}")]
    Sqlx(#[from] SqlxError),
    #[error("Migrate error: {0}")]
    MigrateError(#[from] MigrateError),
    #[error("SeaOrm error: {0}")]
    DbErr(#[from] DbErr),
}

impl ApiError {
    pub fn name(&self) -> String {
        match self {
            Self::Sqlx(x) => x.to_string(),
            Self::MigrateError(x) => x.to_string(),
            Self::DbErr(x) => x.to_string(),
        }
    }
}

//intellij doesn't like this but it compiles
impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::Sqlx(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::MigrateError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::DbErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            code: status_code.as_u16(),
            message: self.to_string(),
            error: self.name(),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
}
