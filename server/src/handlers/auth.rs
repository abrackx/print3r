use actix_web::{HttpResponse, post, get, put, delete, HttpRequest};
use actix_web::web::{Data, Json, Path};
use reqwest::StatusCode;
use sea_orm::{entity::*};
use chrono;

use crate::config::Pool;
use crate::entities::*;
use crate::errors::ApiError;
use crate::handlers::json_response;
use crate::auth::authentication;
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
struct TokenRequest {
    verifier: String,
    auth_code: String
}

#[get("/auth0_callback")]
pub async fn get_token(request: HttpRequest) -> HttpResponse {
    let verifier = request.cookie("AUTH0_VERIFIER").unwrap().value().to_string();
    println!("VERIFIER IN CALLBACK: {}", verifier);
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
    println!("AUTH RES: {}", auth_res.unwrap().text().await.unwrap());
    let response = HttpResponse::Ok().finish();
    return response;
}