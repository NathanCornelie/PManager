pub struct Project {
    pub id: i32,
   pub path: String,
    pub name: String,
    pub description: String,
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
