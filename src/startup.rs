use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::routes::{confirm_sub, health_check};
use crate::routes::subscribe;
use crate::email_client::EmailClient;

pub fn run(listener: TcpListener, 
           conn: PgPool,
           emailc: EmailClient) -> Result<Server, std::io::Error> {
    let conn = web::Data::new(conn);
    let ec = web::Data::new(emailc);
    let server = HttpServer::new(move || App::new()
                                 .wrap(TracingLogger::default())
                                 .route("/health_check", web::get().to(health_check))
                                 .route("/confirm/{request}", web::get().to(confirm_sub))
                                 .route("/subscriptions", web::post().to(subscribe))
                                 .app_data(conn.clone())
                                 .app_data(ec.clone()))
        .listen(listener)?
        .run();
    Ok(server)
}
