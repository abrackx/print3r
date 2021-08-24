use chrono::NaiveDateTime;
use sqlx::Error;

use crate::config::Pool;

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_date: NaiveDateTime,
    pub modified_date: NaiveDateTime,
}

pub async fn get_project_by_uid(db: &Pool, uid: i32) -> Result<Vec<Project>, Error> {
    let res = sqlx::query_as::<_, Project>(
        "
            SELECT projects.*
            FROM projects
            JOIN user_projects ON projects.id = user_projects.project_id
            WHERE user_projects.user_id = $1
    ",
    )
    .bind(uid)
    .fetch_all(db)
    .await?;
    Ok(res)
}

pub async fn get_project_by_id(db: &Pool, project_id: i32) -> Result<Project, Error> {
    let res = sqlx::query_as::<_, Project>(
        "
            SELECT projects.*
            FROM projects
            WHERE id = $1
    ",
    )
    .bind(project_id)
    .fetch_one(db)
    .await?;
    Ok(res)
}
