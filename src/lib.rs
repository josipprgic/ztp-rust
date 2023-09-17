use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse, dev::Server};

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!{"Hello {}!", name}
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/api/health_check", web::get().to(health_check))
    })
        .bind("127.0.0.1:8000")?
    .run();

    Ok(server)
}
