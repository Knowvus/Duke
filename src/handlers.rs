use warp::http::StatusCode;
use warp::Reply;
use warp::reject::Rejection;
use warp::Filter; // Add this line to bring the Filter trait into scope

pub async fn create_task(body: String) -> Result<impl Reply, Rejection> {
    // Logic for reversing the string
    let reversed_string = body.chars().rev().collect::<String>();
    Ok(warp::reply::with_status(reversed_string, StatusCode::OK))
}

pub async fn create_user() -> Result<impl warp::Reply, Rejection> {
    // Logic for creating a user
    Ok(StatusCode::CREATED)
}

pub fn create_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
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
