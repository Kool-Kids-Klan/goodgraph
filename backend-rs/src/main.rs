use actix_web::{post, App, HttpResponse, HttpServer, Responder};

#[post("/health")]
async fn echo() -> impl Responder {
    HttpResponse::Ok().body("healthy")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(echo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
