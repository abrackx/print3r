use actix_web::{HttpResponse, get, post};
use actix_web::web::{Data, Json};
use reqwest::StatusCode;
use sea_orm::{EntityTrait, Set};

use crate::config::Pool;
use crate::entities::posts;
use crate::entities::users;
use crate::errors::ApiError;
use crate::handlers::json_response;

#[get("/posts")]
pub async fn get_all_posts(
    db: Data<Pool>
) -> Result<HttpResponse, ApiError> {
    Ok(json_response(
        //doesn't work as expected
        posts::Entity::find().find_also_related(users::Entity).into_json().all(&db).await.expect("error!"),
        StatusCode::OK,
    ))
}

#[post("/posts")]
pub async fn create_post(
    create_post: Json<posts::Model>,
    db: Data<Pool>
) -> Result<HttpResponse, ApiError> {
    let new_post = posts::ActiveModel {
        name: Set(String::from(&create_post.name)),
        description: Set(String::from(&create_post.description)),
        created_by: Set(create_post.created_by),
        created_date: Set(*&create_post.created_date),
        ..Default::default()
    };
    let res = posts::Entity::insert(new_post).exec(&db).await.expect("o no!");
    Ok(json_response(
        res.last_insert_id,
        StatusCode::OK,
    ))
}
