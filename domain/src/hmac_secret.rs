use secrecy::Secret;
use serde::Deserialize;
use std::ops::Deref;

#[derive(Debug, Clone, Deserialize)]
pub struct HmacSecret(Secret<String>);

impl Deref for HmacSecret {
    type Target = Secret<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for HmacSecret {
    fn from(value: String) -> Self {
        HmacSecret(Secret::new(value))
    }
}
