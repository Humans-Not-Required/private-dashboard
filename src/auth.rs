use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

pub struct ManageKey(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ManageKey {
    type Error = &'static str;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let header = req.headers().get_one("Authorization");
        match header {
            Some(h) if h.starts_with("Bearer ") => {
                Outcome::Success(ManageKey(h[7..].to_string()))
            }
            _ => Outcome::Error((Status::Unauthorized, "Missing or invalid Authorization header")),
        }
    }
}
