mod routes;
mod handlers;
mod db;

use routes::create_routes;
use std::env;
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::SubscriberBuilder;
use tracing_appender::rolling::{RollingFileAppender, Rotation};

#[tokio::main]
async fn main() {
    // Initialize logging
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "app.log");
    let (_non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let subscriber = SubscriberBuilder::default()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Starting the Duke service...");

    // Set up server address
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().expect("Invalid address");

    // Initialize routes and start the server
    let routes = create_routes();
    warp::serve(routes).run(addr).await;

    info!("Server is running on {}", addr);
}