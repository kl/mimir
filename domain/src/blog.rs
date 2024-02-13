use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct BlogPost {
    pub id: i64,
    pub url_id: String,
    pub title: String,
    pub html: String,
    pub markdown: String,
    pub status: BlogPostStatus,
    pub updated_at: Option<DateTime<Utc>>,
}

impl BlogPost {
    pub fn is_published(&self) -> bool {
        matches!(self.status, BlogPostStatus::Published(_))
    }

    pub fn published_at(&self) -> Option<DateTime<Utc>> {
        if let BlogPostStatus::Published(dt) = self.status {
            Some(dt)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub enum BlogPostStatus {
    Published(DateTime<Utc>),
    Unpublished,
}

#[derive(serde::Deserialize, Debug)]
pub struct NewBlogPostData {
    pub title: String,
    pub url_id: String,
    pub markdown: String,
    #[serde(default)]
    pub publish: bool,
}
