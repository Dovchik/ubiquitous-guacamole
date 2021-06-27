use actix_http::Response;
use actix_web::{web, Responder};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    Response::Ok()
}
