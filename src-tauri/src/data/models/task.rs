pub struct Task {
    pub id: i32,
    pub name: String,
    pub description: String,
}

impl Task {
    pub fn new(id: i32, name: String, description: String) -> Self {
        Task {
            id,
            name,
            description,
        }
    }
}
