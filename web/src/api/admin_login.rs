use crate::session::TypedSession;
use crate::web_error::WebError;
use actix_web::error::InternalError;
use actix_web::http::header::LOCATION;
use actix_web::web::Form;
use actix_web::{web, HttpResponse};
use actix_web_flash_messages::FlashMessage;

use domain::AdminUseCase;

use secrecy::Secret;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Login {
    pub password: Secret<String>,
}

#[tracing::instrument(name = "Logging in as admin", skip(form, admin_uc, session))]
pub async fn admin_login(
    Form(form): Form<Login>,
    admin_uc: web::Data<AdminUseCase>,
    session: TypedSession,
) -> Result<HttpResponse, InternalError<WebError>> {
    match admin_uc.validate_admin_credentials(form.password).await {
        Ok(_) => {
            session.set_is_admin(true);
            Ok(HttpResponse::SeeOther()
                .insert_header((LOCATION, "/admin/draft"))
                .finish())
        }
        Err(e) => {
            FlashMessage::error(e.to_string()).send();

            let response = HttpResponse::SeeOther()
                .insert_header((LOCATION, "/admin/login"))
                .finish();

            Err(InternalError::from_response(e.into(), response))
        }
    }
}
