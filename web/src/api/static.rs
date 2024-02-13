use actix_web::HttpResponse;

pub async fn css_dark() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/css; charset=utf-8")
        .body(include_str!("../../css/Solarized (dark)-edit.css"))
}

pub async fn css_light() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/css; charset=utf-8")
        .body(include_str!("../../css/InspiredGitHub-edit.css"))
}

pub async fn css_base() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/css; charset=utf-8")
        .body(include_str!("../../css/water.css"))
}
