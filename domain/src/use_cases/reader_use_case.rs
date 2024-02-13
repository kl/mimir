use crate::{BlogPost, DomainError, Repository};
use chrono::{DateTime, Utc};

pub struct ReaderUseCase {
    repository: Box<dyn Repository>,
}

impl ReaderUseCase {
    pub fn new<R>(repository: R) -> Self
    where
        R: Repository + 'static,
    {
        Self {
            repository: Box::new(repository),
        }
    }

    #[tracing::instrument(name = "Retrieving all published blog posts", skip(self))]
    pub async fn get_published_posts(&self) -> Result<Vec<BlogPost>, DomainError> {
        let mut posts = self
            .repository
            .load_all_posts()
            .await?
            .into_iter()
            .filter(|post| post.is_published())
            .collect::<Vec<_>>();

        posts.sort_unstable_by_key(|post| post.published_at().unwrap_or(DateTime::<Utc>::MIN_UTC));

        Ok(posts)
    }

    #[tracing::instrument(name = "Find blog post by url id", skip(self))]
    pub async fn get_post_by_url_id(&self, url_id: &str) -> Result<Option<BlogPost>, DomainError> {
        let post = self.repository.load_post_by_url_id(url_id).await?;
        Ok(post)
    }
}
