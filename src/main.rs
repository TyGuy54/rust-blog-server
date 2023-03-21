use rocket::*;
use rocket::routes;

mod model;
use model::requests::BlogPostRequest;
use model::responses::BlogPost;
use model::database::{create_blog_post, get_blog_post, get_blog_posts, DBResult};
use rocket::serde::json::Json;
use rocket::State;
use sqlx::{Pool, Sqlite, SqlitePool};

#[post("/create_aritcal", format = "json", data = "<blog_details>")]
async fn create(blog_details: Json<BlogPostRequest>, pool: &State<Pool<Sqlite>>) -> DBResult<Json<BlogPost>> {
    let id = create_blog_post(
        pool, 
        &blog_details.title,  
        &blog_details.author, 
        &blog_details.url, 
        &blog_details.description,
        &blog_details.created_date,
        &blog_details.content).await?;
    let blog_detail = get_blog_post(pool, id).await?;
    Ok(Json(blog_detail))
}

#[get("/view_artical")]
async fn index(pool: &State<Pool<Sqlite>>) -> DBResult<Json<Vec<BlogPost>>> {
    let tasks = get_blog_posts(pool).await?;
    Ok(Json(tasks))
}

#[get("/view_artical/<id>")]
async fn detail(id: i64, pool: &State<Pool<Sqlite>>) -> DBResult<Json<BlogPost>> {
    let task = get_blog_post(pool, id).await?;
    Ok(Json(task))
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let pool = SqlitePool::connect("sqlite://data.db")
        .await
        .expect("Couldn't connect to sqlite database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Couldn't migrate the database tables");

    let _rocket = rocket::build()
        .mount("/", routes![index, create, detail])
        .manage(pool)
        .launch()
        .await?;
    Ok(())
}
