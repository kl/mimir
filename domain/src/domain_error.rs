#[derive(thiserror::Error, Debug)]
pub enum DomainError {
    #[error("{0}")]
    UserValidationError(String),
    #[error("The registration token is not valid")]
    InvalidRegistrationToken,
    #[error("Incorrect admin password")]
    AdminAuthError,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
