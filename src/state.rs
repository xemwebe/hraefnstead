use crate::direction::Direction;
use crate::entity::Entity;
use crate::room::Room;
use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    loc: usize,
    inventory: HashMap<String, Entity>,
    rooms: Vec<Room>,
}

impl State {
    /// Create a very simple game state for testing.
    pub fn new() -> State {
        let mut exit_north = HashMap::new();
        exit_north.insert(Direction::North, 1);
        let mut exit_south = HashMap::new();
        exit_south.insert(Direction::South, 0);
        let mut treasure_aliases = HashSet::new();
        treasure_aliases.insert("coin".to_string());
        treasure_aliases.insert("gold".to_string());
        let treasure = Entity::new("1", "A gold coin", "A shiny gold coin.", treasure_aliases);
        let mut treasure_map = HashMap::new();
        treasure_map.insert(treasure.id.clone(), treasure);

        Self {
            loc: 0,
            rooms: vec![
                Room::new(
                    "Entrance",
                    "You are in the entrance of the dungeon.",
                    HashMap::new(),
                    exit_north,
                ),
                Room::new(
                    "Treasure Room",
                    "You found the treasure room!",
                    treasure_map,
                    exit_south,
                ),
            ],
            inventory: HashMap::new(),
        }
    }

    pub fn get_room(&self) -> &Room {
        &self.rooms[self.loc]
    }

    pub fn get_room_mut(&mut self) -> &mut Room {
        &mut self.rooms[self.loc]
    }

    pub fn get_exit(&self, dir: Direction) -> Option<usize> {
        self.rooms[self.loc].get_exit(dir)
    }

    pub fn set_location(&mut self, new_room: usize) {
        self.loc = new_room;
    }

    pub fn add_to_inventory(&mut self, entity: Entity) {
        self.inventory.insert(entity.id.clone(), entity);
    }

    pub fn get_from_inventory(&mut self, thing: &str) -> Option<Entity> {
        let mut found_id = None;
        for (id, entity) in self.inventory.iter() {
            if entity.aliases.contains(thing) {
                found_id = Some(id.clone());
            }
        }
        if let Some(id) = found_id {
            self.inventory.remove(&id)
        } else {
            println!("You don't have {thing}.");
            None
        }
    }

    pub fn get_inventory(&self) -> &HashMap<String, Entity> {
        &self.inventory
    }
}
