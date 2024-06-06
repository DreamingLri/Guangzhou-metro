use guangzhou_metro::bootstrap;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    bootstrap::app::start().await
}
