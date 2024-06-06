use actix_web::{get, post, HttpResponse, Responder};
use serde_json::json;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[post("/echo")]

pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(serde_json::to_string(&req_body).unwrap())
}
