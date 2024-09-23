use serde::de::{self, MapAccess, SeqAccess};
use serde::{de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub project_id: Option<i32>,
    pub priority: Option<String>,
    pub status: Option<String>,
    pub subtasks: Vec<Subtask>,
}

#[derive(Serialize, Deserialize)]
pub struct Subtask {
    pub id: i32,
    pub value: String,
    pub done: bool,
}

impl Task {
    pub fn new(
        id: i32,
        name: String,
        description: String,
        project_id: Option<i32>,
        priority: Option<String>,
        status: Option<String>,
        subtasks: Vec<Subtask>,
    ) -> Self {
        Task {
            id,
            name,
            description,
            project_id,
            priority,
            status,
            subtasks,
        }
    }

    pub fn to_hash_map(&self) -> HashMap<String, String> {
        let mut h = HashMap::new();
        h.insert("id".to_string(), self.id.to_string());
        h.insert("name".to_string(), self.name.clone());
        h.insert("description".to_string(), self.description.clone());
        h.insert(
            "project_id".to_string(),
            self.project_id.unwrap_or(0).to_string(),
        );
        h.insert(
            "priority".to_string(),
            self.priority.clone().unwrap_or(String::new()).to_string(),
        );
        h.insert(
            "status".to_string(),
            self.status.clone().unwrap_or(String::new()).to_string(),
        );
        
        h
    }
}
