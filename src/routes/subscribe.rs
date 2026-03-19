use actix_web::web;
use actix_web::{HttpRequest, HttpResponse, Responder};

#[derive(serde::Deserialize)]
struct FormData {
    username: String,
}

pub async fn subscribe() -> HttpResponse {
    HttpResponse::Ok().finish()
}
