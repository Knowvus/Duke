use utoipa::OpenApi;
use crate::handlers::{task::create_task, user::create_user};

#[derive(OpenApi)]
#[openapi(
    paths(
        create_task,
        create_user
    ),
    components(
        schemas()
    )
)]
pub struct ApiDoc;
