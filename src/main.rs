use warp::Filter;
use std::net::SocketAddr;
use std::str::FromStr;

mod handlers;
use handlers::{create_routes};

#[tokio::main]
async fn main() {
    println!("Starting the application...");

    let routes = create_routes();

    let addr = SocketAddr::from_str("0.0.0.0:8080").expect("Invalid address");
    println!("Starting server on {}", addr);
    warp::serve(routes)
        .run(addr)
        .await;

    println!("Server has exited."); // This should not normally be reached unless the server stops
}
