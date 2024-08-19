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
            create_task(body)
        });

    create_user_route.or(create_task_route)
}
