use actix_web::{HttpResponse, post, get, put, delete};
use actix_web::web::{Data, Json, Path};
use reqwest::StatusCode;
use sea_orm::{entity::*};
use chrono;

use crate::config::Pool;
use crate::entities::*;
use crate::errors::ApiError;
use crate::handlers::json_response;

#[get("/auth")]
pub async fn start_auth() -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().finish())
}