use chrono::NaiveDateTime;
use sqlx::Error;

use crate::config::Pool;

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_date: NaiveDateTime,
}

pub async fn get_user(db: &Pool) -> Result<Vec<User>, Error> {
    let res = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(db)
        .await?;
    Ok(res)
}

pub async fn get_user_by_id(db: &Pool, uid: i32) -> Result<User, Error> {
    let res = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(uid)
        .fetch_one(db)
        .await?;
    Ok(res)
}
