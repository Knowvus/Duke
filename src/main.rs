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

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}