use utoipa::OpenApi;
use crate::schemas::task::TaskBody;
use crate::handlers::task::create_task; // Import directly if needed
use crate::handlers::user::create_user;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::task::create_task,  // Explicit path to create_task
        crate::handlers::user::create_user   // Explicit path to create_user
    ),
    components(
        schemas(TaskBody)
    )
)]
pub struct ApiDoc;
