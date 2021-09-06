use actix_web::http::StatusCode;
use actix_web::web::{scope, ServiceConfig};
use actix_web::HttpResponse;
use serde::Serialize;
use serde_json::Value;

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
            .service(posts::get_post_by_id)
            .service(posts::get_post_comments)
            .service(posts::create_post)
            .service(posts::update_post)
            .service(posts::delete_post)
            .service(users::get_all_users)
            .service(users::get_user_by_id)
            .service(users::create_user)
            .service(users::update_user)
            .service(users::delete_user),
    );
}

pub fn json_response<T: Serialize>(data: T, status: StatusCode) -> HttpResponse {
    HttpResponse::build(status).json(data)
}

fn replace_json_values(parent: &Value, child: &Option<Value>, key: &str) -> Value {
    let mut new_parent = parent.clone();
    if let Some(replacable) = new_parent.get_mut(key) {
        *replacable = child.clone().unwrap_or(Value::Null);
    }
    new_parent
}

trait MapRelationshipJson {
    fn map_relationship_json(&mut self, key: &str) -> Vec<Value>;
}

impl MapRelationshipJson for Vec<(Value, Option<Value>)> {
    fn map_relationship_json(&mut self, key: &str) -> Vec<Value> {
        return self
            .iter_mut()
            .map(|(parent, child)| replace_json_values(parent, child, key))
            .collect::<Vec<Value>>();
    }
}
