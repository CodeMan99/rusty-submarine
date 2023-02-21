#![feature(decl_macro)]
#[macro_use]
extern crate rocket;

mod apikey;

use apikey::ApiKey;
use rocket::tokio::time::{sleep, Duration};
use rocket::response::status::Accepted;
use rocket::serde::Serialize;
use rocket::serde::json::Json;
use uuid::Uuid;

#[derive(Serialize)]
struct Document {
    id: String,
}

#[post("/typical-create")]
fn typical_create(key: ApiKey) -> Accepted<Json<Document>> {
    println!("      >> Request Accepted with Api-Key: {:?}", key);
    let id = Uuid::new_v4();
    let doc = Document {
        id: id.to_string()
    };
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
    rocket::build().mount("/", routes![
        index,
        goodbye,
        delay,
        typical_create,
    ])
}
