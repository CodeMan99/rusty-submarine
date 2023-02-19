#![feature(decl_macro)]
#[macro_use]
extern crate rocket;

use rocket::tokio::time::{sleep, Duration};

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
    rocket::build().mount("/", routes![index, goodbye, delay])
}
