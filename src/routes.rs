use warp::Filter;
use crate::handlers::{create_task, create_user};

pub fn create_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let create_user_route = warp::path("create_user")
        .and(warp::body::json())
        .and_then(|body: serde_json::Value| {
            let email = body["email"].as_str().unwrap().to_string();
            async move {
                create_user(email).await
            }
        });

    let create_task_route = warp::path("create_task")
        .and(warp::body::json())
        .and_then(|body: serde_json::Value| {
            let task_name = body["task_name"].as_str().unwrap().to_string();
            let created_by = body["created_by"].as_i64().unwrap() as i32;
            async move {
                create_task(task_name, created_by).await
            }
        });

    let health_route = warp::path("health")
        .map(|| warp::reply::with_status("OK", warp::http::StatusCode::OK));

    create_user_route.or(create_task_route).or(health_route)
}
