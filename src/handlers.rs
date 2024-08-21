use warp::http::StatusCode;
use warp::Reply;
use warp::reject::Rejection;

#[utoipa::path(
    post,
    path = "/create_task",
    request_body = TaskBody,
    responses(
        (status = 200, description = "Task created successfully", body = String)
    )
)]
pub async fn create_task(body: String) -> Result<impl Reply, Rejection> {
    let reversed_string = body.chars().rev().collect::<String>();
    Ok(warp::reply::with_status(reversed_string, StatusCode::OK))
}

#[utoipa::path(
    post,
    path = "/create_user",
    responses(
        (status = 201, description = "User created successfully")
    )
)]
pub async fn create_user() -> Result<impl warp::Reply, Rejection> {
    Ok(StatusCode::CREATED)
}
