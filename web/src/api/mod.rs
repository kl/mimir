use domain::BlogPost;

pub(crate) mod admin_login;
pub(crate) mod health_check;
pub(crate) mod new_post;
pub(crate) mod r#static;

pub trait BlogPostAugmentation {
    fn url(&self) -> String;
}

impl BlogPostAugmentation for BlogPost {
    fn url(&self) -> String {
        format!("/blog/{}", &self.url_id)
    }
}
