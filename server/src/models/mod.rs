pub mod file;
pub mod project;
pub mod tag;
pub mod user;

#[derive(Serialize, Deserialize)]
pub struct PageParams {
    pub(crate) limit: Option<i32>,
    pub(crate) offset: Option<i32>,
}
