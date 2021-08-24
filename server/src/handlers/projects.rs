use actix_web::web::{Data, Path};
use actix_web::{get, HttpResponse};
use reqwest::StatusCode;

use crate::config::Pool;
use crate::errors::ApiError;
use crate::handlers::json_response;
use crate::models::project::{get_project_by_id, get_project_by_uid};

#[get("/projects/users/{uid}")]
pub async fn get_projects_by_uid(db: Data<Pool>, uid: Path<i32>) -> Result<HttpResponse, ApiError> {
    Ok(json_response(
        get_project_by_uid(db.get_ref(), uid.into_inner()).await?,
        StatusCode::OK,
    ))
}

#[get("/projects/{project_id}")]
pub async fn get_projects_by_id(
    db: Data<Pool>,
    project_id: Path<i32>,
) -> Result<HttpResponse, ApiError> {
    Ok(json_response(
        get_project_by_id(db.get_ref(), project_id.into_inner()).await?,
        StatusCode::OK,
    ))
}
