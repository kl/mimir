use crate::pages::BlogPostAugmentation;
use crate::web_error::WebError;
use actix_web::{web, HttpResponse};
use askama::Template;
use askama_actix::TemplateToResponse;
use domain::{BlogPost, ReaderUseCase};

#[derive(Template)]
#[template(path = "index.html")]
struct PostsTemplate {
    title: &'static str,
    posts: Vec<BlogPost>,
}

#[tracing::instrument(name = "Serving the admin post page", skip(reader_uc))]
pub async fn blog_posts_page(
    reader_uc: web::Data<ReaderUseCase>,
) -> Result<HttpResponse, WebError> {
    let posts = reader_uc.get_published_posts().await?;
    Ok(PostsTemplate {
        title: "Articles",
        posts,
    }
    .to_response())
}
