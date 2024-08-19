use utoipa::ToSchema;

#[derive(ToSchema)]
pub struct TaskBody {
    pub body: String,
}
