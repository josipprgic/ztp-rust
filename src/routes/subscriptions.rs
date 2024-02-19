use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct SubReq {
    name: String,
    email: String
}


#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(request, connection),
    fields(
        sub_email = %request.email,
        sub_name = %request.name
        )
    )]
pub async fn subscribe(
    request: web::Form<SubReq>, 
    connection: web::Data<PgPool>) -> HttpResponse {
    match insert_sub(&connection, &request).await
        {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => HttpResponse::InternalServerError().finish()  
        }
}

#[tracing::instrument(
    name = "Saving new subscriber to database",
    skip(pool, data)
    )]
pub async fn insert_sub(pool: &PgPool, data: &SubReq) -> Result<(), sqlx::Error> {
    sqlx::query!(r#"
                 insert into subscriptions (email, name, subscribed_at)
                 values ($1, $2, $3)
                 "#,
                 data.email,
                 data.name,
                 Utc::now().naive_utc())
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    Ok(())
}
