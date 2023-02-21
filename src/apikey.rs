use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct ApiKey(Uuid);

#[derive(Debug)]
pub enum ApiKeyError {
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
