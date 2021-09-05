use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, put, HttpResponse};
use reqwest::StatusCode;
use sea_orm::{EntityTrait, Set};
use serde_json::Value;
use crate::config::Pool;
use crate::entities::posts;
use crate::entities::users;
use crate::errors::ApiError;
use crate::handlers::json_response;

#[get("/posts")]
pub async fn get_all_posts(db: Data<Pool>) -> Result<HttpResponse, ApiError> {
    let all_posts = posts::Entity::find()
            .find_with_related(users::Entity)
            .into_json()
            .all(&db)
            .await?;
    Ok(json_response(
        all_posts.iter().map(|x| &x.0).collect::<Vec<&Value>>(),
        StatusCode::OK,
    ))
}

#[post("/posts")]
pub async fn create_post(
    create_post: Json<posts::Model>,
    db: Data<Pool>,
) -> Result<HttpResponse, ApiError> {
    let new_post = posts::ActiveModel {
        name: Set(String::from(&create_post.name)),
        description: Set(String::from(&create_post.description)),
        created_by: Set(create_post.created_by),
        created_date: Set(*&create_post.created_date),
        ..Default::default()
    };
    let res = posts::Entity::insert(new_post)
        .exec(&db)
        .await?;
    Ok(json_response(res.last_insert_id, StatusCode::OK))
}

#[put("/posts/{post_id}")]
pub async fn update_post(
    post_id: Path<i32>,
    update_post: Json<posts::Model>,
    db: Data<Pool>,
) -> Result<HttpResponse, ApiError> {
    let to_update = posts::Entity::find_by_id(post_id.into_inner())
        .one(&db)
        .await?;
    let mut post: posts::ActiveModel = to_update.unwrap().into();
    post.name = Set(update_post.name.to_owned());
    post.description = Set(update_post.description.to_owned());
    let _updated_post = posts::Entity::update(post).exec(&db).await?;
    Ok(HttpResponse::new(StatusCode::ACCEPTED))
}

