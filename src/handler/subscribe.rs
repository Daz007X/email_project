use actix_web::{web,post, Responder,HttpResponse};
use crate::models::form_data::FormData; 

#[post("/subscriptions")]
pub async fn subscriptions_api(_form: web::Form<FormData>) -> impl Responder 
{
    HttpResponse::Ok().finish()
}