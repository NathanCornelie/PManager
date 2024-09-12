use serde::{ser::SerializeStruct, Serialize};

pub struct Task {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub project_id: i32,
}

impl Task {
    pub fn new(id: i32, name: String, description: String, project_id: i32) -> Self {
        Task {
            id,
            name,
            description,
            project_id,
        }
    }
}
impl Serialize for Task {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Task", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("project_id", &self.project_id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("description", &self.description)?;
        state.end()
    }
}
