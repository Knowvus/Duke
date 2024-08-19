use warp::http::StatusCode;
use warp::Reply;
use warp::reject::Rejection;
use utoipa::path;

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
