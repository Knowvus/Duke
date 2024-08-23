use diesel::prelude::*;
use diesel::sql_query;
use warp::http::StatusCode;
use warp::Rejection;
use crate::db::establish_connection;

pub async fn create_user(email: String) -> Result<impl warp::Reply, Rejection> {
    let mut conn = establish_connection();  // Make the connection mutable

    // Raw SQL query to insert a new user
    let query = format!("INSERT INTO user_users (email) VALUES ('{}')", email);
    
    match sql_query(query).execute(&mut conn) {  // Pass the mutable connection
        Ok(_) => Ok(warp::reply::with_status("User created", StatusCode::CREATED)),
        Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)) => {
            Ok(warp::reply::with_status("Email already exists", StatusCode::CONFLICT))
        }
        Err(_) => Ok(warp::reply::with_status("Failed to create user", StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

pub async fn create_task(task_name: String, created_by: i32) -> Result<impl warp::Reply, Rejection> {
    let mut conn = establish_connection();  // Make the connection mutable

    // Raw SQL query to insert a new task
    let query = format!(
        "INSERT INTO framework_tasks (name, created_by) VALUES ('{}', {})",
        task_name, created_by
    );
    
    match sql_query(query).execute(&mut conn) {  // Pass the mutable connection
        Ok(_) => Ok(warp::reply::with_status("Task created", StatusCode::CREATED)),
        Err(_) => Ok(warp::reply::with_status("Failed to create task", StatusCode::INTERNAL_SERVER_ERROR)),
    }
}
