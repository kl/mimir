use crate::web_error::WebError;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::web::{Form, Redirect};
use actix_web::{web, HttpResponse, HttpResponseBuilder, Responder};
use anyhow::anyhow;
use domain::{AdminUseCase, DomainError, NewBlogPostData};

#[tracing::instrument(name = "Creating a new blog post", skip(form, admin_uc))]
pub async fn new_post(
    form: Form<NewBlogPostData>,
    admin_uc: web::Data<AdminUseCase>,
) -> Result<impl Responder, WebError> {
    admin_uc.store_blog_post(&form).await?;
    Ok(Redirect::to(format!("/blog/{}", form.url_id)).see_other())
}

#[derive(serde::Deserialize, Debug)]
pub struct PreviewPostData {
    pub markdown: String,
}

#[tracing::instrument(name = "Generating blog post preview HTML", skip(form, admin_uc))]
pub async fn preview_html(
    form: Form<PreviewPostData>,
    admin_uc: web::Data<AdminUseCase>,
) -> Result<impl Responder, WebError> {
    let html = admin_uc.generate_html(form.0.markdown).await?;
    Ok(HttpResponseBuilder::new(StatusCode::OK)
        .content_type(ContentType::html())
        .body(html))
}

/// Only installed when running integration tests
pub async fn blow_up() -> Result<HttpResponse, WebError> {
    Err(DomainError::UnexpectedError(anyhow!("User registration store error")).into())
}
