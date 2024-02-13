use crate::api::admin_login::admin_login;
use crate::api::health_check::health_check;
use crate::api::new_post::{blow_up, new_post, preview_html};
use crate::api::r#static::{css_base, css_dark, css_light};
use crate::pages::admin_draft_page::draft_post_page;
use crate::pages::index_page::blog_posts_page;
use crate::pages::login_page::login_page;
use crate::pages::view_post_page::view_post_page;
use crate::session::TypedSession;
use actix_session::config::CookieContentSecurity;
use actix_session::storage::CookieSessionStore;
use actix_session::{SessionExt, SessionMiddleware};

use actix_web::cookie::Key;
use actix_web::dev::{Server, Service, ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::error::ErrorInternalServerError;
use actix_web::middleware::Compress;
use actix_web::web::Data;
use actix_web::{web, App, HttpResponse, HttpServer, Scope};
use actix_web_flash_messages::storage::CookieMessageStore;
use actix_web_flash_messages::FlashMessagesFramework;
use domain::{AdminUseCase, HmacSecret, ReaderUseCase};
use secrecy::ExposeSecret;
use std::ffi::OsStr;
use std::future;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub struct ServerArguments {
    pub listener: TcpListener,
    pub reader_use_case: Data<ReaderUseCase>,
    pub admin_use_case: Data<AdminUseCase>,
    pub hmac_secret: HmacSecret,
}

pub fn run_server(
    ServerArguments {
        listener,
        reader_use_case,
        admin_use_case,
        hmac_secret,
    }: ServerArguments,
) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        let key = Key::from(hmac_secret.expose_secret().as_bytes());

        let flash_messages =
            FlashMessagesFramework::builder(CookieMessageStore::builder(key.clone()).build())
                .build();

        let session = SessionMiddleware::builder(CookieSessionStore::default(), key)
            .cookie_content_security(CookieContentSecurity::Private)
            .build();

        let mut app = App::new()
            .wrap(Compress::default())
            .wrap(session)
            .wrap(flash_messages)
            .wrap(TracingLogger::default())
            .route("/", web::get().to(blog_posts_page))
            .route("login", web::get().to(login_page))
            .route("login", web::post().to(admin_login))
            .route("/health_check", web::get().to(health_check))
            .service(web::resource("/blog/{post}").route(web::get().to(view_post_page)))
            .service(
                web::scope("static")
                    .route("light.css", web::get().to(css_light))
                    .route("dark.css", web::get().to(css_dark))
                    .route("base.css", web::get().to(css_base)),
            )
            .service(
                authorized_scope("admin")
                    .route("draft", web::get().to(draft_post_page))
                    .route("new_post", web::post().to(new_post))
                    .route("preview_html", web::post().to(preview_html)),
            )
            .app_data(reader_use_case.clone())
            .app_data(admin_use_case.clone());

        if is_running_integration_test_or_benchmark() {
            app = app.route("/blow_up", web::get().to(blow_up));
        }
        app
    })
    .listen(listener)?
    .run();
    Ok(server)
}

// All routes attached to this scope will return Unauthorized if the user isn't
// logged in as the admin.
fn authorized_scope(
    scope: &str,
) -> Scope<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    web::scope(scope).wrap_fn(|req, service| {
        let session: TypedSession = req.get_session().into();

        return match session.is_admin() {
            Ok(true) => service.call(req),
            Ok(false) => Box::pin(future::ready(Ok(ServiceResponse::new(
                req.request().clone(),
                HttpResponse::Unauthorized().finish(),
            )))),
            Err(err) => Box::pin(future::ready(Ok(ServiceResponse::new(
                req.request().clone(),
                HttpResponse::from_error(ErrorInternalServerError(err)),
            )))),
        };
    })
}

fn is_running_integration_test_or_benchmark() -> bool {
    // Make sure we are being run through Cargo
    if std::env::var_os("CARGO_PKG_NAME").is_none() {
        return false;
    }

    // If the currently executing exe is in the `deps` dir
    // this means we are running integration tests or benchmarks.
    std::env::current_exe()
        .map(|exe| {
            exe.parent()
                .and_then(std::path::Path::file_name)
                .map(|parent_name| parent_name == OsStr::new("deps"))
                .unwrap_or(false)
        })
        .unwrap_or(false)
}
