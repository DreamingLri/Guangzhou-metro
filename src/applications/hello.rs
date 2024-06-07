use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[derive(Deserialize)]
struct EchoRequest {
    message: String,
}

#[post("/echo")]
pub async fn echo(info: web::Form<EchoRequest>) -> impl Responder {
    HttpResponse::Ok().body(info.message.clone())
}
