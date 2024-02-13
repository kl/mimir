mod api;
mod pages;
pub(crate) mod session;
pub mod startup;
pub(crate) mod web_error;

pub use actix_web::dev::Server;
pub use actix_web::web::Data;

#[cfg(feature = "dev-server")]
pub static DEV_SCRIPTS: &'static str = concat!(
    "<script>\n",
    include_str!("../templates/reload.js"),
    "</script>\n"
);

#[cfg(not(feature = "dev-server"))]
pub static DEV_SCRIPTS: &str = "";

pub static ROUTE_API_LOGIN: &str = "/login";
pub static ROUTE_API_NEW_POST: &str = "/admin/new_post";
pub static ROUTE_API_PREVIEW_HTML: &str = "/admin/preview_html";
