use warp::Filter;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let reverse = warp::post()
        .and(warp::body::json())
        .map(|body: Value| {
            let input = body["input_string"].as_str().unwrap_or("");
            let reversed: String = input.chars().rev().collect();
            warp::reply::json(&serde_json::json!({ "reversed_string": reversed }))
        });

    warp::serve(reverse).run(([0, 0, 0, 0], 3030)).await;
}