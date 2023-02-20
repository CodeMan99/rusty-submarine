#![feature(decl_macro)]
#[macro_use]
extern crate rocket;

use rocket::tokio::time::{sleep, Duration};
use uuid::Uuid;
use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};

#[derive(Debug, Clone, Copy)]
struct ApiKey(Uuid);

#[derive(Debug)]
enum ApiKeyError {
    Missing,
    Invalid(&'static str),
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("x-api-key").map(Uuid::parse_str) {
            None => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            Some(Ok(key)) => Outcome::Success(ApiKey(key)),
            Some(Err(_err)) => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid("Unable to parse API-KEY")))
        }
    }
}

#[post("/typical-create")]
fn typical_create(key: ApiKey) -> String {
    println!("      >> Request Accepted with Api-Key: {:?}", key);
    format!("Some value {}\r\n", Uuid::new_v4())
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
