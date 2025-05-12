use log::info;
use rust_actix_juniper_graphql::server::server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::init_logging();
    info!("Starting GraphQL server");
    server::start_server().await
}
