use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

async fn greet_name(path: web::Path<String>) -> impl Responder {
    format!("Hello, {}!", path)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/greet/{name}", web::get().to(greet_name))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
