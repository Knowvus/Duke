mod routes;
mod apidoc;
mod handlers;
mod schemas;

use routes::create_routes;
use std::env;
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::SubscriberBuilder;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use warp::Filter;

#[tokio::main]
async fn main() {
    // Create a rolling file appender
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "app.log");
    let (_non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // Initialize tracing subscriber with file appender
    let subscriber = SubscriberBuilder::default()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Starting the application...");

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().expect("Invalid address");

    let routes = create_routes();
    let swagger_ui = warp::path("docs").and(warp::fs::dir("swagger-ui/"));

    let all_routes = routes.or(swagger_ui);

    info!("Starting server on {}", addr);

    // Run the server
    warp::serve(all_routes).run(addr).await;

    info!("Server has exited.");
}