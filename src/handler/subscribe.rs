use actix_web::{web,post, Responder,HttpResponse};
use crate::models::form_data::FormData; 
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;

#[post("/subscriptions")]
pub async fn subscriptions_api(form: web::Form<FormData>,pool :web::Data<PgPool>) -> impl Responder
{
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    ).execute(pool.get_ref()).await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
 
    
}