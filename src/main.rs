#![feature(decl_macro)]
#[macro_use]
extern crate rocket;

mod apikey;

use apikey::ApiKey;
use rocket::response::status::Accepted;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::tokio::time::{sleep, Duration};
use rocket_db_pools::{sqlx, Database};
use uuid::Uuid;

#[derive(Database)]
#[database("rusty_db")]
struct RustyDb(sqlx::PgPool);

#[derive(Debug, Serialize)]
struct User {
    id: i64,
    username: String,
    first: Option<String>,
    last: Option<String>,
    age: Option<i32>,
}

#[get("/find-user?<username>")]
async fn find_user(db: &RustyDb, username: &str) -> Option<Json<User>> {
    let maybe_user = sqlx::query_as!(
        User,
        "SELECT id, username, first, last, age FROM public.user WHERE username = $1;",
        username
    )
    .fetch_optional(&db.0)
    .await;

    match maybe_user {
        | Ok(Some(user)) => Some(Json(user)),
        | Ok(None) => None,
        | Err(err) => {
            eprint!("{}", err);
            None
        },
    }
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
    rocket::build()
        .attach(RustyDb::init())
        .mount("/", routes![index, goodbye, delay, typical_create, find_user])
}
