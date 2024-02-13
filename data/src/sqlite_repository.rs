use anyhow::{anyhow, Context};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::{BlogPost, BlogPostStatus, NewBlogPostData, Repository};
use secrecy::Secret;
use sqlx::SqlitePool;
use std::error::Error;
use std::fmt::Display;

#[derive(Clone)]
pub struct SqliteRepository {
    pool: SqlitePool,
}

impl SqliteRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository for SqliteRepository {
    async fn update_admin_password(&self, hashed_password: &str) -> anyhow::Result<()> {
        let mut transaction = self.pool.begin().await?;

        sqlx::query!(r#"DELETE FROM admin_auth"#)
            .execute(&mut *transaction)
            .await
            .error("Failed to delete existing admin password")?;

        sqlx::query!(
            r#"
            INSERT INTO admin_auth (hashed_password)
            VALUES ($1)
            "#,
            hashed_password,
        )
        .execute(&mut *transaction)
        .await
        .error("Failed to insert admin password into the database")?;

        transaction.commit().await?;
        Ok(())
    }

    async fn load_admin_password(&self) -> anyhow::Result<Secret<String>> {
        let pw = Secret::new(
            sqlx::query!(
                r#"
                SELECT hashed_password FROM admin_auth
                "#,
            )
            .fetch_one(&self.pool)
            .await
            .error("Failed to load admin password from the database")?
            .hashed_password,
        );

        Ok(pw)
    }

    async fn store_blog_post(&self, new_post: &NewBlogPostData, html: &str) -> anyhow::Result<()> {
        let published_at = if new_post.publish {
            Some(Utc::now().timestamp())
        } else {
            None
        };

        sqlx::query!(
            r#"
            INSERT INTO posts (url_id, title, markdown, html, is_published, published_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            new_post.url_id,
            new_post.title,
            new_post.markdown,
            html,
            new_post.publish,
            published_at
        )
        .execute(&self.pool)
        .await
        .error("Failed to insert new blog post into the database")?;

        Ok(())
    }

    async fn load_all_posts(&self) -> anyhow::Result<Vec<BlogPost>> {
        let records = sqlx::query_as!(
            BlogPostRecord,
            r#"
            SELECT * FROM posts
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .error("Failed to read blog post from the database")?;

        Ok(records
            .into_iter()
            .map(|r| r.try_into())
            .collect::<anyhow::Result<Vec<BlogPost>>>()?)
    }

    async fn load_post_by_url_id(&self, url_id: &str) -> anyhow::Result<Option<BlogPost>> {
        let record = sqlx::query_as!(
            BlogPostRecord,
            r#"
            SELECT * FROM posts WHERE url_id = ?
            "#,
            url_id
        )
        .fetch_optional(&self.pool)
        .await
        .error("Failed to read blog post from the database")?;

        let post = match record {
            None => None,
            Some(record) => Some(record.try_into()?),
        };

        Ok(post)
    }
}

struct BlogPostRecord {
    pub id: i64,
    pub url_id: String,
    pub title: String,
    pub html: String,
    pub markdown: String,
    pub is_published: i64,
    pub published_at: Option<i64>,
    pub updated_at: Option<i64>,
}

impl TryInto<BlogPost> for BlogPostRecord {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<BlogPost, Self::Error> {
        let status = if self.is_published == 1 {
            BlogPostStatus::Published(
                self.published_at
                    .ok_or_else(|| anyhow!("is_published is true but published_at is null"))?
                    .to_datetime_utc()
                    .context("published_at contains invalid data")?,
            )
        } else {
            BlogPostStatus::Unpublished
        };

        let updated_at = if let Some(ts) = self.updated_at {
            Some(
                ts.to_datetime_utc()
                    .context("updated_at contains invalid data")?,
            )
        } else {
            None
        };

        Ok(BlogPost {
            id: self.id,
            url_id: self.url_id,
            title: self.title,
            html: self.html,
            markdown: self.markdown,
            status,
            updated_at,
        })
    }
}

trait ErrorHelper<T> {
    fn error<C>(self, context: C) -> anyhow::Result<T>
    where
        C: Display + Send + Sync + 'static;

    fn unit_error<C>(self, context: C) -> anyhow::Result<()>
    where
        C: Display + Send + Sync + 'static;
}

impl<T, E: Send + Sync + Error + 'static> ErrorHelper<T> for Result<T, E> {
    fn error<C>(self, context: C) -> anyhow::Result<T>
    where
        C: Display + Send + Sync + 'static,
    {
        self.map_err(|e| anyhow!(e).context(context))
    }

    fn unit_error<C>(self, context: C) -> anyhow::Result<()>
    where
        C: Display + Send + Sync + 'static,
    {
        self.map_err(|e| anyhow!(e).context(context)).map(|_| ())
    }
}

trait ConvertHelper {
    fn to_datetime_utc(self) -> anyhow::Result<DateTime<Utc>>;
}

impl ConvertHelper for i64 {
    fn to_datetime_utc(self) -> anyhow::Result<DateTime<Utc>> {
        DateTime::<Utc>::from_timestamp(self, 0)
            .ok_or(anyhow!("Could not convert into timestamp: {}", self))
    }
}
