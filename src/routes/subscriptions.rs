use actix_web::{web, HttpResponse, ResponseError};
use anyhow::Context;
use chrono::Utc;
use reqwest::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{domain::SubscriberDetails, email_client::EmailClient};

#[derive(serde::Deserialize)]
pub struct SubReq {
    pub name: String,
    pub email: String,
}

#[derive(thiserror::Error)]
pub enum SubscribeError {
    #[error("{0}")]
    InputValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ResponseError for SubscribeError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InputValidationError(_) => StatusCode::BAD_REQUEST,
            Self::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl std::fmt::Debug for SubscribeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

#[tracing::instrument(
    name = "Confirming subscription",
    skip(request, connection),
    fields(token = %request)
    )]
pub async fn confirm_sub(
    request: web::Path<Uuid>,
    connection: web::Data<PgPool>,
) -> Result<HttpResponse, SubscribeError> {
    sqlx::query!(
        r#"
                    UPDATE subscriptions SET status = 'confirmed'
                    WHERE id IN (SELECT id FROM confirmation_tokens WHERE token = $1)
                  "#,
        request.as_ref()
    )
    .execute(connection.as_ref())
    .await
    .context("Failed to change subscriber status")?;

    Ok(HttpResponse::Ok().finish())
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
    email_client: web::Data<EmailClient>,
) -> Result<HttpResponse, SubscribeError> {
    let sub = request
        .0
        .try_into()
        .map_err(|s| SubscribeError::InputValidationError(s))?;

    let token = insert_sub(&connection, &sub)
        .await
        .context("Failed to insert subscriber")?;

    email_client
        .send(
            sub.email,
            format!("Welcome {}, we need just one more thing", sub.name.as_ref()),
            format!("curl localhost:8000/confirm/{}", token),
            "Confirm yourself maan",
        )
        .await
        .context("Failed to send confirmation email")?;

    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(name = "Saving new subscriber to database", skip(pool, data))]
async fn insert_sub(pool: &PgPool, data: &SubscriberDetails) -> Result<Uuid, sqlx::Error> {
    let conf_token = Uuid::new_v4();

    sqlx::query!(
        r#"
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
        conf_token
    )
    .execute(pool)
    .await?;

    Ok(conf_token)
}
