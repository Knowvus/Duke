use std::env;
use std::net::SocketAddr;
use std::str::FromStr;
use warp::Filter;

mod routes;
mod apidoc;
mod handlers;
mod schemas;

use routes::create_routes;

#[tokio::main]
async fn main() {
    println!("Starting the application...");

    // Ensure the PORT environment variable is set, or exit with an error
    let port = env::var("PORT").expect("PORT environment variable not set");
    let addr = SocketAddr::from_str(&format!("0.0.0.0:{}", port))
        .expect("Invalid address");

    println!("Starting server on {}", addr);

    // Create the warp routes for your API
    let routes = create_routes();

    // Set up the Swagger UI
    let swagger_ui = warp::path("docs").and(warp::fs::dir("swagger-ui/"));

    // Combine both services to be run concurrently
    tokio::join!(
        warp::serve(routes).run(addr),
        warp::serve(swagger_ui).run(addr)
    );

    println!("Server has exited.");
}
