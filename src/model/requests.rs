use rocket::serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct BlogPostRequest {
    pub title: String,
    pub url: String,
    pub author: String,
    pub created_date: String,
    pub description: String,
    pub content: String
}