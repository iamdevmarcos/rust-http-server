mod config;
mod server;
mod routes;
mod handlers;
mod models;

#[tokio::main]
async fn main() {
    let config = config::Config::load();
    print!("🔥 Server running at {}:{}", config.host, config.port);

    server::start(config).await;
}
