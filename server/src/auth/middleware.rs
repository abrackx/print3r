use actix_web::error::ErrorUnauthorized;
use actix_web::{dev, Error, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use sea_orm::{entity::*, QueryFilter};
use actix_web::web::Data;
use crate::config::Pool;
use crate::entities::prelude::Users;
use std::pin::Pin;
use std::future::Future;
use crate::entities::users;

pub struct AuthRoute {
    pub user: users::Model,
}

#[derive(Serialize, Deserialize)]
pub struct SelfResponse {
    email: String,
}

impl FromRequest for AuthRoute {
    type Error = Error;
    type Future = Pin<Box< dyn Future<Output = Result<AuthRoute, Error>>>>;
    type Config = ();

    fn from_request(_req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        return match _req.cookie("ACCESS_TOKEN") {
            None => { Box::pin(async { Err(ErrorUnauthorized("Invalid token.")) }) }
            Some(access_token) => {
                let db = _req.app_data::<Data<Pool>>().unwrap().clone();
                Box::pin(async move {
                    let client = reqwest::Client::new();
                    let res = client.get("https://dev-05tizgpa.us.auth0.com/userinfo")
                        .header("Authorization", format!("Bearer {}", String::from(access_token.value())))
                        .send().await;
                    return match res {
                        Ok(res) => {
                            let email = res.json::<SelfResponse>().await.unwrap().email;
                            let user = users::Entity::find()
                                .filter(users::Column::Email.contains(email.as_str()))
                                .one(&db).await;
                            match user {
                                Ok(user) => {
                                    return match user {
                                        Some(user) => {
                                            Ok(AuthRoute { user })
                                        }
                                        None => {
                                            Err(ErrorUnauthorized("Invalid token."))
                                        }
                                    };
                                }
                                Err(..) => { Err(ErrorUnauthorized("Invalid token.")) }
                            }
                        }
                        Err(..) => { Err(ErrorUnauthorized("Invalid token")) }
                    }
                })
            }
        };
    }
}