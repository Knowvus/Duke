use std::env;
use std::net::SocketAddr;
use warp::Filter;

mod routes;
mod apidoc;
mod handlers;
mod schemas;

use routes::create_routes;

#[tokio::main]
async fn main() {
    println!("Starting the application...");

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().expect("Invalid address");

    let routes = create_routes();
    let swagger_ui = warp::path("docs").and(warp::fs::dir("swagger-ui/"));

    println!("Starting server on {}", addr);

    tokio::join!(
        warp::serve(routes).run(addr),
        warp::serve(swagger_ui).run(addr)
    );

    println!("Server has exited.");
}
