use actix_session::{Session, SessionExt, SessionGetError};
use actix_web::dev::Payload;

use actix_web::{FromRequest, HttpRequest};
use std::future;
use std::future::Ready;

pub struct TypedSession(Session);

impl TypedSession {
    const IS_ADMIN_KEY: &'static str = "is_admin";

    pub fn set_is_admin(&self, is_admin: bool) {
        self.0.insert(Self::IS_ADMIN_KEY, is_admin).unwrap();
    }

    pub fn is_admin(&self) -> Result<bool, SessionGetError> {
        self.0
            .get::<bool>(Self::IS_ADMIN_KEY)
            .map(|r| r.unwrap_or(false))
    }
}

impl FromRequest for TypedSession {
    type Error = <Session as FromRequest>::Error;
    // From request expects a `Future` as return type to allow for extractors
    // that need to perform asynchronous operations (e.g. a HTTP call)
    // We do not have a `Future`, because we don't perform any I/O,
    // so we wrap `TypedSession` into `Ready` to convert it into a `Future` that
    // resolves to the wrapped value the first time it's polled by the executor.
    type Future = Ready<Result<TypedSession, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        future::ready(Ok(TypedSession(req.get_session())))
    }
}

impl From<Session> for TypedSession {
    fn from(val: Session) -> Self {
        TypedSession(val)
    }
}
