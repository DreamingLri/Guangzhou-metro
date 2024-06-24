use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

#[get("/getStation")]
pub async fn get_station() -> impl Responder {
    let stations = crate::get_map();
    let line = stations.line_stations();
    HttpResponse::Ok().json(line)
}

#[derive(Deserialize)]
struct LineRequest {
    start: String,
    end: String,
}

#[post("getLine")]
pub async fn get_line(info: web::Json<LineRequest>) -> impl Responder {
    let map = crate::get_map();
    let path = map.find_path(&info.start, &info.end);
    HttpResponse::Ok().json(path)
}
