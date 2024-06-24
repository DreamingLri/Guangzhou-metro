use actix_cors::Cors;
use actix_web::{App, HttpServer};

use crate::applications;

pub async fn start() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .service(applications::hello::hello)
            .service(applications::hello::echo)
            .service(applications::stations::get_line)
            .service(applications::stations::get_station)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
