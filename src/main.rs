use warp::Filter;
use warp::http::StatusCode;
use warp::Reply;
mod handlers;
use handlers::{create_task, create_routes};

#[tokio::main]
async fn main() {
    let create_task_route = warp::path("create_task")
        .and_then(create_task);

    let routes = create_task_route;

    use std::net::SocketAddr;
    use std::str::FromStr;
    
    let addr = SocketAddr::from_str("0.0.0.0:8080").expect("Invalid address");
    warp::serve(routes)
        .run(addr)
        .await;
}
