#![feature(decl_macro)]
#[macro_use]
extern crate rocket;

mod apikey;

use apikey::ApiKey;
use rocket::response::status::Accepted;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::time::{sleep, Duration};
use rocket_db_pools::{sqlx, Database};
use uuid::Uuid;

#[derive(Database)]
#[database("rusty_db")]
struct RustyDb(sqlx::PgPool);

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i64,
    username: String,
    first: Option<String>,
    last: Option<String>,
    age: Option<i32>,
}

#[get("/find-user?<username>")]
async fn find_user(
    db: &RustyDb,
    username: &str,
) -> Result<Option<Json<User>>, rocket::http::Status> {
    let result = sqlx::query_as!(
        User,
        "SELECT id, username, first, last, age FROM public.user WHERE username = $1;",
        username
    )
    .fetch_optional(&db.0)
    .await;

    match result {
        Ok(Some(user)) => Ok(Some(Json(user))),
        Ok(None) => Ok(None),
        Err(err) => {
            eprintln!("{}", err);
            Err(rocket::http::Status { code: 500 })
        }
    }
}

#[post("/create-user", data = "<user>")]
async fn create_user(
    key: ApiKey,
    db: &RustyDb,
    user: Json<User>,
) -> Option<&'static str> {
    println!("      >> Request Accepted with Api-Key: {:?}", key);

    let result = sqlx::query_as!(
        User,
        "INSERT INTO public.user(username, first, last, age) VALUES ($1, $2, $3, $4);",
        user.username,
        user.first,
        user.last,
        user.age,
    )
    .execute(&db.0)
    .await;

    result.ok().map(|_| "Success")
}

#[derive(Serialize)]
struct Document {
    id: Uuid,
}

#[post("/typical-create")]
fn typical_create(key: ApiKey) -> Accepted<Json<Document>> {
    println!("      >> Request Accepted with Api-Key: {:?}", key);
    let id = Uuid::new_v4();
    let doc = Document { id };
    Accepted(Some(Json(doc)))
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds\r\n", seconds)
}

#[get("/")]
fn index() -> &'static str {
    "Hello Cody\r\n"
}

#[get("/goodbye")]
fn goodbye() -> &'static str {
    "Goodbye Cody\r\n"
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(RustyDb::init()).mount(
        "/",
        routes![index, goodbye, delay, typical_create, find_user, create_user],
    )
}
