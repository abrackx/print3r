use actix_web::{HttpResponse, post, get, put, delete, HttpRequest};
use actix_web::web::{Data, Json, Path};
use reqwest::StatusCode;
use sea_orm::{entity::*, QueryFilter};
use chrono;

use crate::config::Pool;
use crate::entities::*;
use crate::errors::ApiError;
use crate::handlers::json_response;
use crate::auth::authentication;
use crate::auth::middleware::AuthRoute;
use actix_web::cookie::Cookie;
use std::borrow::Borrow;
use serde_json::de::Read;

#[get("/auth")]
pub async fn start_auth() -> HttpResponse {
    let verifier = authentication::generate_verifier();
    let challenge = authentication::generate_challenge(verifier.as_str().parse().unwrap());
    let redirect_url = authentication::get_auth_redirect_url(challenge.as_str().parse().unwrap());

    let cookie = Cookie::build("AUTH0_VERIFIER", verifier.as_str())
        .http_only(true)
        .path("/")
        .finish();

    let response = HttpResponse::PermanentRedirect()
        .cookie(cookie)
        .append_header(("location", redirect_url)).finish();
    return response;
}

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    access_token: String,
    id_token: String,
    expires_in: i32,
    token_type: String,
}

#[get("/auth0_callback")]
pub async fn get_token(request: HttpRequest) -> HttpResponse {
    let verifier = request.cookie("AUTH0_VERIFIER").unwrap().value().to_string();
    let auth_code = request.query_string().split("=").nth(1).unwrap().to_string();
    let req_body = [
        ("grant_type", "authorization_code".to_string()),
        ("code_verifier", verifier),
        ("code", auth_code),
        ("client_id", "xodFBsdfd2LQXzzaqac3979dnE8GhcEq".to_string()),
        ("client_secret", "NI_R_Q22rxqHX43uUlPxsxH5DgYBSWaTdnYLmXX_0TWdjoFBq9eQ8K-4lRas8z9V".to_string()),
        ("redirect_uri", "http://localhost:8888/api/v1/auth0_callback".to_string())
    ];
    let client = reqwest::Client::new();
    let auth_res = client
        .post("https://dev-05tizgpa.us.auth0.com/oauth/token")
        .form(&req_body)
        .send()
        .await;
    let cookie = Cookie::build("ACCESS_TOKEN", auth_res.unwrap().json::<AuthResponse>().await.unwrap().access_token)
        .http_only(true)
        .path("/")
        .finish();
    let response = HttpResponse::PermanentRedirect()
        .cookie(cookie)
        .append_header(("location", "/api/v1/self"))
        .finish();
    return response;
}

#[derive(Serialize, Deserialize)]
pub struct SelfResponse {
    email: String,
}

#[get("/self")]
pub async fn get_self(_: AuthRoute, request: HttpRequest, db: Data<Pool>) -> Result<HttpResponse, ApiError> {
    let access_token = request.cookie("ACCESS_TOKEN").unwrap().value().to_string();
    let client = reqwest::Client::new();
    let res = client.get("https://dev-05tizgpa.us.auth0.com/userinfo")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await;
    let email = res.unwrap().json::<SelfResponse>().await.unwrap().email;
    let user: Option<serde_json::value::Value> = users::Entity::find()
        .filter(users::Column::Email.contains(email.as_str()))
        .into_json()
        .one(&db)
        .await
        .expect("error!");
    Ok(json_response(user, StatusCode::OK))
}