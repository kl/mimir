use actix_web::HttpResponse;
use actix_web_flash_messages::{IncomingFlashMessages, Level};
use askama::Template;
use askama_actix::TemplateToResponse;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate<'a> {
    title: &'a str,
    errors: Vec<&'a str>,
}

#[tracing::instrument(name = "Showing admin login page", skip(flash))]
pub async fn login_page(flash: IncomingFlashMessages) -> HttpResponse {
    LoginTemplate {
        title: "Admin login",
        errors: flash
            .iter()
            .filter(|m| m.level() == Level::Error)
            .map(|m| m.content())
            .collect(),
    }
    .to_response()
}
