use actix_web::http::StatusCode;
use actix_web::web::{Data, Path, Query};
use actix_web::{get, HttpResponse};

use crate::config::Pool;
use crate::errors::ApiError;
use crate::handlers::json_response;
use crate::models::tag::get_tag;
use crate::models::PageParams;

#[get("/tags")]
pub async fn get_tags(db: Data<Pool>, params: Query<PageParams>) -> Result<HttpResponse, ApiError> {
    let page = params.into_inner();
    Ok(json_response(
        get_tag(db.get_ref(), page.limit, page.offset).await?,
        StatusCode::OK,
    ))
}
