use warp::Filter;
use warp::http::StatusCode;
use warp::Reply;

pub async fn create_task() -> Result<impl Reply, warp::Rejection> {
    // Logic for creating a task
    Ok(StatusCode::CREATED)
}

pub async fn create_user() -> Result<impl warp::Reply, warp::Rejection> {
    // Logic for creating a user
    Ok(StatusCode::CREATED)
}

pub fn create_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let create_user_route = warp::path("create_user")
        .and_then(create_user);

    let create_task_route = warp::path("create_task")
        .and_then(create_task);

    create_user_route.or(create_task_route)
}