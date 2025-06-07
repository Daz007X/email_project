use actix_web::{get, HttpResponse};

#[get("/")]
pub async fn health_check_api() -> HttpResponse {
    HttpResponse::Ok().body("OK มาก ๆ")
}