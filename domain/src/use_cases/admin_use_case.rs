use crate::{markdown, util, BlogPost, DomainError, NewBlogPostData, Repository};
use anyhow::{anyhow, Context};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use secrecy::{ExposeSecret, Secret};

pub struct AdminUseCase {
    repository: Box<dyn Repository>,
}

impl AdminUseCase {
    pub fn new<R>(repository: R) -> Self
    where
        R: Repository + 'static,
    {
        Self {
            repository: Box::new(repository),
        }
    }

    #[tracing::instrument(name = "Retrieving all blog posts", skip(self))]
    pub async fn get_all_posts(&self) -> Result<Vec<BlogPost>, DomainError> {
        let posts = self.repository.load_all_posts().await?;

        Ok(posts)
    }

    #[tracing::instrument(name = "Storing a new blog post", skip(self, new_blog))]
    pub async fn store_blog_post(&self, new_blog: &NewBlogPostData) -> Result<(), DomainError> {
        let html = self.generate_html(new_blog.markdown.clone()).await?;
        self.repository.store_blog_post(new_blog, &html).await?;
        Ok(())
    }

    #[tracing::instrument(name = "Generating HTML for markdown", skip(self, markdown))]
    pub async fn generate_html(&self, markdown: String) -> Result<String, DomainError> {
        let html = util::spawn_blocking_with_tracing(move || markdown::convert_to_html(&markdown))
            .await
            .context("Failed to spawn blocking task")
            .map_err(DomainError::UnexpectedError)??;

        Ok(html)
    }

    #[tracing::instrument(
        name = "Checking if given admin password is correct",
        skip(self, password)
    )]
    pub async fn validate_admin_credentials(
        &self,
        password: Secret<String>,
    ) -> Result<(), DomainError> {
        let stored_password = self.repository.load_admin_password().await?;
        let correct = util::spawn_blocking_with_tracing(move || {
            identical_passwords(stored_password, password)
        })
        .await
        .context("Failed to spawn blocking task")
        .map_err(DomainError::UnexpectedError)??;

        if correct {
            Ok(())
        } else {
            Err(DomainError::AdminAuthError)
        }
    }
}

#[tracing::instrument(name = "Verify password hash", skip(stored, given))]
fn identical_passwords(stored: Secret<String>, given: Secret<String>) -> anyhow::Result<bool> {
    let stored_hash = PasswordHash::new(stored.expose_secret().as_str())
        .map_err(|e| anyhow!(e))
        .context("Failed to parse hash in PHC string format")
        .map_err(DomainError::UnexpectedError)?;

    Ok(Argon2::default()
        .verify_password(given.expose_secret().as_bytes(), &stored_hash)
        .is_ok())
}
