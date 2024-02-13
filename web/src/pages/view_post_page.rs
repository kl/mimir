use crate::web_error::WebError;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse};
use askama::Template;
use askama_actix::TemplateToResponse;
use domain::{BlogPost, ReaderUseCase};

#[derive(Template)]
#[template(path = "view_post.html")]
struct PostTemplate<'a> {
    title: &'a str,
    post: &'a BlogPost,
}

#[tracing::instrument(name = "Viewing post", skip(reader_uc))]
pub async fn view_post_page(
    reader_uc: web::Data<ReaderUseCase>,
    id: web::Path<String>,
) -> Result<HttpResponse, WebError> {
    if let Some(post) = reader_uc.get_post_by_url_id(&id.into_inner()).await? {
        Ok(PostTemplate {
            title: &post.title,
            post: &post,
        }
        .to_response())
    } else {
        Ok(HttpResponse::new(StatusCode::NOT_FOUND))
    }
}
