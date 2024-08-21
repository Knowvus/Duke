use utoipa::ToSchema;

#[derive(ToSchema)]
pub struct TaskBody {
    pub body: String,
}

impl TaskBody {
    pub fn read_body(&self) {
        println!("{}", self.body);
    }
}
