use chrono::NaiveDateTime;
use sqlx::Error;

use crate::config::Pool;

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub created_date: NaiveDateTime,
}

pub async fn get_tag(
    db: &Pool,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<Tag>, Error> {
    let res = sqlx::query_as::<_, Tag>(
        "
            SELECT * 
            FROM tags 
            ORDER BY id 
            LIMIT $1 
            OFFSET $2 
        ",
    )
    .bind(limit.unwrap_or(25))
    .bind(offset.unwrap_or(0))
    .fetch_all(db)
    .await?;
    Ok(res)
}
