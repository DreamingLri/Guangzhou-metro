use actix_web::{get, App, HttpResponse, HttpServer, Responder};

pub async fn start() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}
