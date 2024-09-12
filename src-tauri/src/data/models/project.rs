use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

pub struct Project {
    pub id: i32,
   pub path: String,
    pub name: String,
    pub description: String,
}
impl Serialize for Project{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Project", 4)?;
        state.serialize_field("id",&self.id)?;
        state.serialize_field("path", &self.path)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("description", &self.description)?;
        state.end()
    }
}


pub struct ProjectCreate {
    pub path: String,
    pub name: String,
    pub description: String,

}
impl Project {
    pub fn new(id: i32,path: String, name: String, description: String) -> Self {
        Project {
            id,
            path,
            name,
            description
        }
    }
    pub fn create(path: String, name: String, description: String) -> ProjectCreate{
        ProjectCreate{
            path,
            name,
            description
        }
    }
}
