use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn healt_check() -> impl Responder {
    HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/healt_check", web::get().to(healt_check)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
