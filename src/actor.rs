use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use gag::BufferRedirect;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Actor {
    pub name: String,
    pub description: String,
    pub aliases: HashSet<String>,
}

impl Actor {
    pub fn new(name: &str, description: &str, aliases: HashSet<String>) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            aliases,
        }
    }
}
