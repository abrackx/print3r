use crate::config::Pool;
use crate::entities::comments;
use crate::errors::ApiError;
use crate::handlers::json_response;
use actix_web::web::{Data, Json, Path};
use actix_web::{delete, post, put, HttpResponse};
use reqwest::StatusCode;
use sea_orm::{EntityTrait, Set};

#[post("/comments")]
pub async fn create_comment(
    create_comment: Json<comments::CreateComment>,
    db: Data<Pool>,
) -> Result<HttpResponse, ApiError> {
    let new_comment = comments::ActiveModel {
        post_id: Set(create_comment.post_id),
        content: Set(String::from(&create_comment.content)),
        created_by: Set(create_comment.created_by),
        created_date: Set(chrono::offset::Utc::now().naive_utc()),
        ..Default::default()
    };
    let res = comments::Entity::insert(new_comment).exec(&db).await?;
    Ok(json_response(res.last_insert_id, StatusCode::ACCEPTED))
}

#[put("/comments/{comment_id}")]
pub async fn update_comment(
    comment_id: Path<i32>,
    update_comment: Json<comments::CreateComment>,
    db: Data<Pool>,
) -> Result<HttpResponse, ApiError> {
    let to_update = comments::Entity::find_by_id(comment_id.into_inner())
        .one(&db)
        .await?;
    let mut comment: comments::ActiveModel = to_update.ok_or(ApiError::NotFound)?.into();
    comment.content = Set(update_comment.content.to_owned());
    let _updated_comment = comments::Entity::update(comment).exec(&db).await?;
    Ok(HttpResponse::new(StatusCode::ACCEPTED))
}

#[delete("/comments/{comment_id}")]
pub async fn delete_comment(comment_id: Path<i32>, db: Data<Pool>) -> Result<HttpResponse, ApiError> {
    let to_delete = comments::Entity::find_by_id(comment_id.into_inner())
        .one(&db)
        .await?;
    let comment: comments::ActiveModel = to_delete.ok_or(ApiError::NotFound)?.into();
    let _deleted_comment = comments::Entity::delete(comment).exec(&db).await?;
    Ok(HttpResponse::new(StatusCode::ACCEPTED))
}
