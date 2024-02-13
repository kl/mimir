use actix_web::HttpResponse;
use askama::Template;
use askama_actix::TemplateToResponse;

#[derive(Template)]
#[template(path = "admin_draft.html")]
struct HelloTemplate<'a> {
    title: &'a str,
}

#[tracing::instrument(name = "Serving the admin post page")]
pub async fn draft_post_page() -> HttpResponse {
    HelloTemplate {
        title: "Admin - New Post",
    }
    .to_response()
}
