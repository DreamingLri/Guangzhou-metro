use guangzhou_metro::bootstrap;

//starter
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    bootstrap::app::start().await
}