use crate::DomainError;
use anyhow::anyhow;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use argon2::{Algorithm, Params, Version};
use secrecy::{ExposeSecret, Secret};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone)]
pub struct Password(Secret<String>);

const PASSWORD_MIN_LENGTH: usize = 8;
const PASSWORD_MAX_LENGTH: usize = 255;

impl Password {
    pub fn parse(s: Secret<String>) -> Result<Password, DomainError> {
        let length = s.expose_secret().graphemes(true).count();

        if length < PASSWORD_MIN_LENGTH {
            return Err(DomainError::UserValidationError(format!(
                "password must be at least #{PASSWORD_MIN_LENGTH} characters long."
            )));
        } else if length > PASSWORD_MAX_LENGTH {
            return Err(DomainError::UserValidationError(format!(
                "password must be at most #{PASSWORD_MAX_LENGTH} characters long."
            )));
        }
        {
            Ok(Self(s))
        }
    }

    pub fn hash_password(&self) -> anyhow::Result<String> {
        let password = self.expose_password().as_bytes();
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::DEFAULT);

        let hashed = argon2
            .hash_password(password, &salt)
            .map_err(|e| anyhow!("{e}"))?
            .to_string();
        Ok(hashed)
    }
}

impl Password {
    pub(crate) fn expose_password(&self) -> &str {
        self.0.expose_secret()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claim::{assert_err, assert_ok};

    #[test]
    fn empty_is_rejected() {
        let password = Secret::new("".to_string());
        assert_err!(Password::parse(password));
    }

    #[test]
    fn too_short_is_rejected() {
        let password = Secret::new("pass123".to_string());
        assert_err!(Password::parse(password));
    }

    #[test]
    fn too_long_is_rejected() {
        let password = Secret::new("w00t".repeat(100));
        assert_err!(Password::parse(password));
    }

    #[test]
    fn valid_is_accepted() {
        let password = Secret::new("password123".to_string());
        assert_ok!(Password::parse(password));
    }
}
