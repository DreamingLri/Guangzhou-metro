use actix_web::{get, App, HttpServer};

use crate::applications;

pub async fn start() -> std::io::Result<()> {
    HttpServer::new(|| 
        App::new().service(applications::hello::hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

