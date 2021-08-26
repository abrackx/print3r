use actix_web::http::StatusCode;
use actix_web::web::{scope, ServiceConfig};
use actix_web::HttpResponse;
use serde::Serialize;

mod posts;
mod users;

pub fn init(cfg: &mut ServiceConfig) {
    info!(
        "Configuring services for {:?}...",
        std::thread::current().id()
    );
    cfg.service(
        scope("/api/v1")
            .service(posts::get_all_posts)
            .service(posts::create_post)
            .service(users::create_user)
    );
}

pub fn json_response<T: Serialize>(data: T, status: StatusCode) -> HttpResponse {
    HttpResponse::build(status).json(data)
}
