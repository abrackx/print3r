use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Sqlx error: {0}")]
    Sqlx(#[from] SqlxError),
}

impl ApiError {
    pub fn name(&self) -> String {
        match self {
            Self::Sqlx(x) => x.to_string(),
        }
    }
}

//intellij doesn't like this but it compiles
impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::Sqlx(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
