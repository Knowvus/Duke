use warp::http::StatusCode;
use warp::Reply;
use warp::reject::Rejection;

pub fn create_task(body: String) -> Result<impl Reply, Rejection> {
    let reversed_string = body.chars().rev().collect::<String>();
    Ok(warp::reply::with_status(reversed_string, StatusCode::OK))
}

pub async fn create_user() -> Result<impl warp::Reply, Rejection> {
    Ok(StatusCode::CREATED)
}