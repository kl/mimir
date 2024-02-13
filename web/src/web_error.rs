use actix_web::http::StatusCode;
use actix_web::ResponseError;
use domain::DomainError;

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct WebError(#[from] DomainError);

impl ResponseError for WebError {
    fn status_code(&self) -> StatusCode {
        use DomainError::*;
        match self.0 {
            UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AdminAuthError => StatusCode::UNAUTHORIZED,
            _ => StatusCode::BAD_REQUEST,
        }
    }
}
