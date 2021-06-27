use actix_http::Response;
use actix_web::Responder;

pub async fn health_check() -> impl Responder {
    Response::Ok()
}
