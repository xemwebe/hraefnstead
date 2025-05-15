use crate::direction::Direction;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    pub name: String,
    pub description: String,
    pub entities: HashSet<usize>,
    pub actors: HashSet<usize>,
    pub exits: HashMap<Direction, usize>,
}

impl Room {
    pub fn new(
        name: &str,
        description: &str,
        entities: HashSet<usize>,
        actors: HashSet<usize>,
        exits: HashMap<Direction, usize>,
    ) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            entities,
            actors,
            exits,
        }
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_exits(&self) -> &HashMap<Direction, usize> {
        &self.exits
    }

    pub fn get_exit(&self, dir: Direction) -> Option<usize> {
        self.exits.get(&dir).copied()
    }

    pub fn get_entities(&self) -> &HashSet<usize> {
        &self.entities
    }

    pub fn remove_entity(&mut self, entity_id: usize) -> bool {
        self.entities.remove(&entity_id)
    }

    pub fn add_entity(&mut self, entity_id: usize) {
        self.entities.insert(entity_id);
    }

    pub fn get_actors(&self) -> &HashSet<usize> {
        &self.actors
    }
    pub fn remove_actor(&mut self, actor_id: usize) -> bool {
        self.actors.remove(&actor_id)
    }
    pub fn add_exit(&mut self, direction: Direction, room_number: usize) {
        self.exits.insert(direction, room_number);
    }
}
