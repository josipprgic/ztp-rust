use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

use crate::routes::health_check;
use crate::routes::subscribe;

pub fn run(listener: TcpListener, conn: PgPool) -> Result<Server, std::io::Error> {
    let conn = web::Data::new(conn);
    let server = HttpServer::new(move || App::new()
                                 .route("/health_check", web::get().to(health_check))
                                 .route("/subscriptions", web::post().to(subscribe))
                                 .app_data(conn.clone()))
        .listen(listener)?
        .run();
    Ok(server)
}
