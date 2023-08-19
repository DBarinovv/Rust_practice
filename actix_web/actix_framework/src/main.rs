use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/greet", web::get().to(health_check))
                    .route("/greet/{name}", web::get().to(greet_name)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
