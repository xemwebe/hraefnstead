use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Entity {
    pub id: String,
    pub name: String,
    pub description: String,
    pub aliases: HashSet<String>,
}

impl Entity {
    pub fn new(id: &str, name: &str, description: &str, aliases: HashSet<String>) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            aliases,
        }
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
