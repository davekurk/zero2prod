use actix_web::{web, App, HttpResponse, HttpServer, Responder, dev::Server};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

// We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation.
pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
        .route("/health_checks", web::get().to(health_check))
        })
        .bind("127.0.0.1:8000")?
        .run();
        // No .await here
    Ok(server)
}
