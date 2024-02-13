use actix_web::HttpResponse;

use std::sync::OnceLock;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body(random_token())
}

fn random_token() -> String {
    static RANDOM: OnceLock<u64> = OnceLock::new();
    let n = *RANDOM.get_or_init(rand::random);
    format!("{n}")
}
