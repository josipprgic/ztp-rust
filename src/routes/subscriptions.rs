use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use chrono::Utc;

use crate::domain::SubscriberDetails;

#[derive(serde::Deserialize)]
pub struct SubReq {
    pub name: String,
    pub email: String
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
    let sub = match request.0.try_into() {
        Ok(s) => s,
        Err(e) => {
            tracing::error!(e);
            return HttpResponse::BadRequest().finish()
        },
    };

    match insert_sub(&connection, &sub).await
        {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => HttpResponse::InternalServerError().finish()  
        }
}

#[tracing::instrument(
    name = "Saving new subscriber to database",
    skip(pool, data)
    )]
pub async fn insert_sub(pool: &PgPool, data: &SubscriberDetails) -> Result<(), sqlx::Error> {
    sqlx::query!(r#"
                 insert into subscriptions (email, name, subscribed_at)
                 values ($1, $2, $3)
                 "#,
                 data.email.as_ref(),
                 data.name.as_ref(),
                 Utc::now().naive_utc())
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    Ok(())
}
