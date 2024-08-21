use std::future;
use warp::Filter;
use crate::handlers::{create_task, create_user};

pub fn create_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let create_user_route = warp::path("create_user")
    .and_then(create_user);

    let create_task_route = warp::path("create_task")
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::bytes())
        .and_then(|body: bytes::Bytes| {
            let body = String::from_utf8(body.to_vec()).unwrap_or_default();
            future::ready(create_task(body))
        });

    let health_route = warp::path("health")
        .map(|| warp::reply::with_status("OK", warp::http::StatusCode::OK));

    // Removed the `docs_route` definition and `ApiDoc` struct

    create_user_route.or(create_task_route).or(health_route)
}
