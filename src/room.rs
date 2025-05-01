use crate::direction::Direction;
use crate::entity::Entity;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    pub name: String,
    pub description: String,
    pub entities: HashMap<String, Entity>,
    pub exits: HashMap<Direction, usize>,
}

impl Room {
    pub fn new(
        name: &str,
        description: &str,
        entities: HashMap<String, Entity>,
        exits: HashMap<Direction, usize>,
    ) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            entities,
            exits,
        }
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_entities(&self) -> &HashMap<String, Entity> {
        &self.entities
    }

    pub fn get_exit(&self, dir: Direction) -> Option<usize> {
        self.exits.get(&dir).map(|&x| x)
    }

    pub fn get_exits(&self) -> &HashMap<Direction, usize> {
        &self.exits
    }

    pub fn get_entity(&mut self, thing: &String) -> Option<Entity> {
        let mut found_entry = None;
        for entry in self.entities.iter() {
            if entry.1.aliases.contains(thing) {
                found_entry = Some(entry.0.clone());
                break;
            }
        }
        if let Some(entry) = found_entry {
            self.entities.remove(&entry)
        } else {
            None
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity.id.clone(), entity);
    }
}
