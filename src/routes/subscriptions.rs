use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;

use crate::{domain::SubscriberDetails, email_client::EmailClient};

#[derive(serde::Deserialize)]
pub struct SubReq {
    pub name: String,
    pub email: String
}

#[tracing::instrument(
    name = "Confirming subscription",
    skip(request, connection),
    fields(token = %request)
    )]
pub async fn confirm_sub(request: web::Path<Uuid>,
                         connection: web::Data<PgPool>) -> HttpResponse {
     match sqlx::query!(r#"
                    UPDATE subscriptions SET status = 'confirmed'
                    WHERE id IN (SELECT id FROM confirmation_tokens WHERE token = $1)
                  "#,
                 request.as_ref())
        .execute(connection.as_ref())
        .await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(e) => {
                tracing::error!("{}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(request, connection, email_client),
    fields(
        sub_email = %request.email,
        sub_name = %request.name
        )
    )]
pub async fn subscribe(
    request: web::Form<SubReq>, 
    connection: web::Data<PgPool>,
    email_client: web::Data<EmailClient>) -> HttpResponse {
    let sub = match request.0.try_into() {
        Ok(s) => s,
        Err(e) => {
            tracing::error!(e);
            return HttpResponse::BadRequest().finish()
        },
    };

    match insert_sub(&connection, &sub).await
        {
            Ok(token) => match email_client.send(
                sub.email, 
                format!("Welcome {}, we need just one more thing", sub.name.as_ref()), 
                format!("curl localhost:8000/confirm/{}", token), 
                "Confirm yourself maan").await {
                Ok(_) => HttpResponse::Ok().finish(),
                Err(_) => HttpResponse::InternalServerError().finish()
            }
            Err(_) => HttpResponse::InternalServerError().finish()  
        }
}

#[tracing::instrument(
    name = "Saving new subscriber to database",
    skip(pool, data)
    )]
pub async fn insert_sub(pool: &PgPool, data: &SubscriberDetails) -> Result<Uuid, sqlx::Error> {
    let conf_token = Uuid::new_v4();

    sqlx::query!(r#"
                 WITH rows AS (
                    INSERT INTO subscriptions(email, name, subscribed_at)
                    VALUES($1, $2, $3)
                    RETURNING id
                )
                INSERT INTO confirmation_tokens(id, token)
                    SELECT id, $4
                    FROM rows;
                 "#,
                 data.email.as_ref(),
                 data.name.as_ref(),
                 Utc::now().naive_utc(),
                 conf_token)
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    Ok(conf_token)
}
