use actix_web::{HttpResponse, post};
use actix_web::web::{Data, Json};
use reqwest::StatusCode;
use sea_orm::{entity::*};

use crate::config::Pool;
use crate::entities::*;
use crate::errors::ApiError;
use crate::handlers::json_response;

#[post("/users")]
pub async fn create_user(
    create_user: Json<users::Model>,
    db: Data<Pool>,
) -> Result<HttpResponse, ApiError> {
    let new_user = users::ActiveModel {
        first_name: Set(String::from(&create_user.first_name)),
        last_name: Set(String::from(&create_user.last_name)),
        email: Set(String::from(&create_user.email)),
        created_date: Set(*&create_user.created_date),
        ..Default::default()
    };
    let x = users::Entity::insert(new_user).exec(&db).await.expect("oops");
    Ok(json_response(
        x.last_insert_id,
        StatusCode::CREATED,
    ))
}
