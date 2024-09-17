use serde::de::{self, MapAccess, SeqAccess};
use serde::{de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::fmt;

pub struct Task {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub project_id: Option<i32>,
    pub priority: Option<String>,
}

impl Task {
    pub fn new(
        id: i32,
        name: String,
        description: String,
        project_id: Option<i32>,
        priority: Option<String>,
    ) -> Self {
        Task {
            id,
            name,
            description,
            project_id,
            priority,
        }
    }

    pub fn to_HashMap(&self) -> HashMap<String, String> {
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
        h
    }
}
impl<'de> Deserialize<'de> for Task {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Id,
            Name,
            Description,
            ProjectId,
            Status,
        }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str(
                            "`id` or `name` or `description` or `projectId` or `priority`",
                        )
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "id" => Ok(Field::Id),
                            "name" => Ok(Field::Name),
                            "description" => Ok(Field::Description),
                            "project_id" => Ok(Field::ProjectId),
                            "priority" => Ok(Field::Status),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }
        struct TaskVisitor;

        impl<'de> Visitor<'de> for TaskVisitor {
            type Value = Task;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Task")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Task, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let id = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let name = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let description = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let project_id = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;
                let priority = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(4, &self))?;
                Ok(Task::new(id, name, description, project_id, priority))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Task, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut id = None;
                let mut name = None;
                let mut description = None;
                let mut project_id = None;
                let mut priority = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Id => {
                            if id.is_some() {
                                return Err(de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        }
                        Field::Name => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        Field::Description => {
                            if description.is_some() {
                                return Err(de::Error::duplicate_field("description"));
                            }
                            description = Some(map.next_value()?);
                        }
                        Field::ProjectId => {
                            if project_id.is_some() {
                                return Err(de::Error::duplicate_field("project_id"));
                            }
                            project_id = Some(map.next_value()?);
                        }
                        Field::Status => {
                            if priority.is_some() {
                                return Err(de::Error::duplicate_field("priority"));
                            }
                            priority = Some(map.next_value()?);
                        }
                    }
                }
                let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
                let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
                let description =
                    description.ok_or_else(|| de::Error::missing_field("description"))?;
                let project_id =
                    project_id.ok_or_else(|| de::Error::missing_field("project_id"))?;
                let priority = priority.ok_or_else(|| de::Error::missing_field("priority"))?;
                Ok(Task::new(id, name, description, project_id, priority))
            }
        }

        const FIELDS: &[&str] = &["secs", "nanos"];
        deserializer.deserialize_struct("Task", FIELDS, TaskVisitor)
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
        state.serialize_field("priority", &self.priority)?;
        state.end()
    }
}
