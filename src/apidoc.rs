use utoipa::OpenApi;
use crate::schemas::TaskBody;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::create_task,
        crate::handlers::create_user
    ),
    components(
        schemas(TaskBody)
    )
)]
pub struct ApiDoc;
