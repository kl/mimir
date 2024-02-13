use crate::blog::BlogPost;
use crate::NewBlogPostData;
use async_trait::async_trait;
use secrecy::Secret;

#[async_trait]
pub trait Repository: Send + Sync {
    async fn update_admin_password(&self, hashed_password: &str) -> anyhow::Result<()>;
    async fn load_admin_password(&self) -> anyhow::Result<Secret<String>>;
    async fn store_blog_post(&self, new_post: &NewBlogPostData, html: &str) -> anyhow::Result<()>;
    async fn load_all_posts(&self) -> anyhow::Result<Vec<BlogPost>>;
    async fn load_post_by_url_id(&self, url_id: &str) -> anyhow::Result<Option<BlogPost>>;
}
