use actix_web::{get, HttpResponse};
use actix_web::web::Data;
use reqwest::StatusCode;
use sea_orm::EntityTrait;

use crate::config::Pool;
use crate::entities::posts::Entity as Posts;
use crate::errors::ApiError;
use crate::handlers::json_response;

#[get("/posts")]
pub async fn get_all_posts(
    db: Data<Pool>
) -> Result<HttpResponse, ApiError> {
    Ok(json_response(
        Posts::find().into_json().all(&db).await.expect("error!"),
        StatusCode::OK,
    ))
}
