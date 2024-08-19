use warp::http::StatusCode;
use warp::reject::Rejection;

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
