use sqlx::{Pool, Sqlite};
use super::responses::BlogPost;

pub type DBResult<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

pub async fn create_blog_post(
    pool: &Pool<Sqlite>,
    title: &String,
    author: &String,
    url: &String,
    created_date: &String,
    description: &String,
    content: &String
) -> DBResult<i64> {
    let mut connection = pool.acquire().await?;
    let id = sqlx::query_as!(
        BlogPost,
        r#"
        INSERT INTO blog_posts (
            title, 
            author, 
            created_date,
            url, 
            description, 
            content) 
        VALUES (?, ?, ?, ?, ?, ?);
        "#,
            title,
            author,
            url,
            created_date,
            description,
            content
    )
        .execute(&mut connection)
        .await?
        .last_insert_rowid();
        Ok(id)
}

pub async fn get_blog_post(pool: &Pool<Sqlite>, id: i64) -> DBResult<BlogPost> {
    let mut connection = pool.acquire()
        .await?;
    let task = sqlx::query_as!(
        BlogPost,
        r#"
        SELECT id, title, author, url, created_date, description, content from blog_posts
        WHERE id = ?;
        "#,
            id
    )
        .fetch_one(&mut connection)
        .await?;
        Ok(task)
}

pub async fn get_blog_posts(pool: &Pool<Sqlite>) -> DBResult<Vec<BlogPost>> {
    let mut connection = pool.acquire()
        .await
        .unwrap();
    let tasks = sqlx::query_as::<_, BlogPost>(
        r#"
        SELECT id, title, author, url, description, content from blog_posts;
        "#
    )
        .fetch_all(&mut connection)
        .await?;
        Ok(tasks)
}
    