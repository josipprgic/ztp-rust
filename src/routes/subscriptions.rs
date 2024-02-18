use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct SubReq {
    name: String,
    email: String
}

pub async fn subscribe(_request: web::Form<SubReq>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
