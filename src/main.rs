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

    // Create the warp routes for your API
    let routes = create_routes();

    // Set up the Swagger UI
    let swagger_ui = warp::path("docs").and(warp::fs::dir("swagger-ui/"));

    // Combine both services to be run concurrently
    let addr = SocketAddr::from_str("0.0.0.0:8080").expect("Invalid address");
    println!("Starting server on {}", addr);

    // Run the API and Swagger UI on different routes
    tokio::join!(
        warp::serve(routes).run(addr),
        warp::serve(swagger_ui).run(addr)
    );

    println!("Server has exited.");
}
