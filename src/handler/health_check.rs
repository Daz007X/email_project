use actix_web::{get, Responder,HttpResponse};

#[get("/health_check")]
pub async fn health_check_api() -> impl Responder {
    HttpResponse::Ok()
}