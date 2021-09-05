use actix_web::http::StatusCode;
use actix_web::web::{scope, ServiceConfig};
use actix_web::HttpResponse;
use serde::Serialize;

mod posts;
mod users;
mod auth;

pub fn init(cfg: &mut ServiceConfig) {
    info!(
        "Configuring services for {:?}...",
        std::thread::current().id()
    );
    cfg.service(
        scope("/api/v1")
            .service(posts::get_all_posts)
            .service(posts::create_post)
            .service(users::get_all_users)
            .service(users::get_user_by_id)
            .service(users::create_user)
            .service(users::update_user)
            .service(users::delete_user)
            .service(auth::start_auth)
            .service(auth::get_token)
    );
}

pub fn json_response<T: Serialize>(data: T, status: StatusCode) -> HttpResponse {
    HttpResponse::build(status).json(data)
}
