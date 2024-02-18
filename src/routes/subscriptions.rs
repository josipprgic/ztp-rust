use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use chrono::Utc;

#[derive(serde::Deserialize)]
pub struct SubReq {
    name: String,
    email: String
}

pub async fn subscribe(
    request: web::Form<SubReq>, 
    connection: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(r#"
                 insert into subscriptions (email, name, subscribed_at)
                 values ($1, $2, $3)
                 "#,
                 request.email,
                 request.name,
                 Utc::now().naive_utc())
        .execute(connection.get_ref())
        .await
        {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(e) => {
                println!("Failed to execute query: {}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
}
