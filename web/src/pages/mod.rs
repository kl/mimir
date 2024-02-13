use domain::BlogPost;

pub(crate) mod admin_draft_page;
pub(crate) mod index_page;
pub(crate) mod login_page;
pub(crate) mod view_post_page;

pub trait BlogPostAugmentation {
    fn url(&self) -> String;
    fn published_at_string(&self) -> String;
}

impl BlogPostAugmentation for BlogPost {
    fn url(&self) -> String {
        format!("/blog/{}", &self.url_id)
    }

    fn published_at_string(&self) -> String {
        self.published_at()
            .map(|dt| dt.to_string())
            .unwrap_or_default()
    }
}
