use actix_web::http::StatusCode;
use actix_web::web::{Data, Path};
use actix_web::{get, HttpResponse};

use crate::config::Pool;
use crate::errors::ApiError;
use crate::handlers::json_response;
use crate::models::user::{get_user, get_user_by_id};

#[get("/users")]
pub async fn get_users(db: Data<Pool>) -> Result<HttpResponse, ApiError> {
    Ok(json_response(get_user(db.get_ref()).await?, StatusCode::OK))
}

#[get("/users/{uid}")]
pub async fn get_users_by_id(db: Data<Pool>, uid: Path<i32>) -> Result<HttpResponse, ApiError> {
    Ok(json_response(
        get_user_by_id(db.get_ref(), uid.into_inner()).await?,
        StatusCode::OK,
    ))
}
