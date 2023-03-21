use rocket::serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow, Debug)]
#[serde(crate = "rocket::serde")]
pub struct BlogPost {
    pub id: i64,
    pub title: String,
    pub url: String,
    pub author: String,
    pub created_date: String,
    pub description: String,
    pub content: String
}