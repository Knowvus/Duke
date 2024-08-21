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

#[tokio::main]
async fn main() {
    // Create a rolling file appender
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "app.log");
    let (_non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // Initialize tracing subscriber with the file appender
    let subscriber = SubscriberBuilder::default()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Starting the application...");

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().expect("Invalid address");

    let routes = create_routes();
    
    warp::serve(routes).run(addr).await;

    info!("Starting server on {}", addr);

    // // Run the server
    // if let Err(e) = warp::serve(all_routes).run(addr).await {
    //     error!("Server error: {}", e);
    // }

    info!("Server has exited.");
}
