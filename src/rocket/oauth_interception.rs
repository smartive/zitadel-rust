use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use crate::credentials::Application;

// TODO: confirm naming.
enum AuthorityAuthentication {
    Basic { client_id: String, client_secret: String },
    JWTProfile { application: Application },
}

pub struct InterceptionConfig {
    authority: String,
    authentication: AuthorityAuthentication,
}

pub struct InterceptionUser {
    // todo.
}

enum InterceptionError {
    Unauthenticated,
    Inactive,
}

impl<'request> FromRequest<'request> for InterceptionUser {
    type Error = InterceptionError;

    async fn from_request(request: &'request Request<'_>) -> Outcome<Self, Self::Error> {
        todo!()
    }
}
