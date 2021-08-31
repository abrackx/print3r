use actix_web::{HttpResponse, post, get};
use actix_web::web::{Data, Json, Path};
use reqwest::StatusCode;
use sea_orm::{entity::*};
use chrono;

use crate::config::Pool;
use crate::entities::*;
use crate::errors::ApiError;
use crate::handlers::json_response;

#[get("/users")]
pub async fn get_all_users(
    db: Data<Pool>
) -> Result<HttpResponse, ApiError> {
    let users: Vec<serde_json::value::Value> = users::Entity::find()
        .into_json()
        .all(&db)
        .await
        .expect("error!");
    Ok(json_response(users, StatusCode::OK))
}

#[get("/users/{user_id}")]
pub async fn get_user_by_id(
    user_id: Path<u32>,
    db: Data<Pool>,
) -> Result<HttpResponse, ApiError> {
    let user: Option<serde_json::value::Value> = users::Entity::find_by_id(user_id.into_inner())
        .into_json()
        .one(&db)
        .await
        .expect("error!");
    Ok(json_response(user, StatusCode::OK))
}

#[post("/users")]
pub async fn create_user(
    create_user: Json<users::CreateForm>,
    db: Data<Pool>,
) -> Result<HttpResponse, ApiError> {
    let new_user = users::ActiveModel {
        first_name: Set(String::from(&create_user.first_name)),
        last_name: Set(String::from(&create_user.last_name)),
        email: Set(String::from(&create_user.email)),
        created_date: Set(chrono::offset::Utc::now().naive_utc()),
        ..Default::default()
    };
    let x = users::Entity::insert(new_user).exec(&db).await.expect("oops");
    Ok(json_response(
        x.last_insert_id,
        StatusCode::CREATED,
    ))
}
