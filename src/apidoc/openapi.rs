use utoipa::OpenApi;
use crate::schemas::task::TaskBody;

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
