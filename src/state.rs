use crate::actor::Actor;
use crate::command::Command;
use crate::condition::Condition;
use crate::direction::Direction;
use crate::entity::Entity;
use crate::event::Event;
use crate::room::Room;

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    loc: usize,
    inventory: HashSet<usize>,
    rooms: Vec<Room>,
    entities: HashMap<usize, Entity>,
    actors: HashMap<usize, Actor>,
    active_events: HashSet<usize>,
    events: Vec<Event>,
    conditions: Vec<Condition>,
    file_name: String,
}

impl State {
    /// Create a very simple game state for testing.
    pub fn new() -> State {
        let mut exits_entrance = HashMap::new();
        exits_entrance.insert(Direction::North, 1);
        let mut exits_corridor = HashMap::new();
        exits_corridor.insert(Direction::South, 0);
        exits_corridor.insert(Direction::East, 2);
        let mut exits_chamber = HashMap::new();
        exits_chamber.insert(Direction::West, 1);
        let mut exits_treasure_room = HashMap::new();
        exits_treasure_room.insert(Direction::South, 2);

        let mut treasure_aliases = HashSet::new();
        treasure_aliases.insert("pile".to_string());
        treasure_aliases.insert("stack".to_string());
        treasure_aliases.insert("gold".to_string());
        let treasure = Entity::new(
            "A stack of gold",
            "It consists for roughly some hundred shiny gold coins.",
            treasure_aliases,
        );

        let mut copper_aliases = HashSet::new();
        copper_aliases.insert("copper".to_string());
        copper_aliases.insert("coin".to_string());
        let coin = Entity::new(
            "A copper coin",
            "An old, dirty copper coin.",
            copper_aliases,
        );

        let mut vending_machine_aliases = HashSet::new();
        vending_machine_aliases.insert("machine".to_string());
        vending_machine_aliases.insert("vending".to_string());
        let vending_machine = Entity::new("A vending machine", "The vending machine has a small slid for the coins. The display is too dirty to reveal what it sells.", vending_machine_aliases);

        let mut entity_map = HashMap::new();
        entity_map.insert(1, treasure);
        entity_map.insert(2, coin);
        entity_map.insert(3, vending_machine);

        let mut actors_map = HashMap::new();
        let mut goblin_aliases = HashSet::new();
        goblin_aliases.insert("goblin".to_string());
        actors_map.insert(
            1,
            Actor::new(
                "Goblin",
                "A small green goblin leans against a door to the north.",
                goblin_aliases,
            ),
        );

        let mut chamber_actors = HashSet::new();
        chamber_actors.insert(1);

        let mut treasure_entities = HashSet::new();
        treasure_entities.insert(1);

        let mut entrance_entities = HashSet::new();
        entrance_entities.insert(3);

        let conditions = vec![
            Condition::Location(2),
            Condition::CommandIs(Command::Examine("bed".to_string())),
            Condition::And(0, 1),
        ];

        let events = vec![
            Event::new(2,
                "The bed is made of soft wood and has a comfortable mattress. Below the pillow you find a copper coin".to_string(),
                vec![Command::AddItemToRoom(2), Command::DeActivateEvent(0), Command::ActivateEvent(1)]),
            Event::new(2,
                "Now that you have taken the coin, you glance down at an empty bed".to_string(),
                vec![]
            )
        ];

        let mut active_events = HashSet::new();
        active_events.insert(0);
        

        Self {
            loc: 0,
            rooms: vec![
                Room::new(
                    "Entrance",
                    "You are in the entrance of the dungeon.",
                    entrance_entities,
                    HashSet::new(),
                    exits_entrance,
                ),
                Room::new(
                    "Corridor",
                    "You are in a dark corridor.",
                    HashSet::new(),
                    HashSet::new(),
                    exits_corridor,
                ),
                Room::new(
                    "Chamber",
                    "There is a bed in the chamber. The pillows are soft.",
                    HashSet::new(),
                    chamber_actors,
                    exits_chamber,
                ),
                Room::new(
                    "Treasure Room",
                    "You found the treasure room!",
                    treasure_entities,
                    HashSet::new(),
                    exits_treasure_room,
                ),
            ],
            inventory: HashSet::new(),
            entities: entity_map,
            actors: actors_map,
            active_events,
            events,
            conditions,
            file_name: super::SAVE_FILE.to_string(),
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

    pub fn get_from_inventory(&mut self, thing: &str) -> Option<(usize, &Entity)> {
        let mut found_entity = None;
        for id in self.inventory.iter() {
            if let Some(entity) = self.entities.get(id) {
                if entity.aliases.contains(thing) {
                    found_entity = Some((*id, entity));
                }
            }
        }
        if let Some((id, entity)) = found_entity {
            if self.inventory.remove(&id) {
                Some((id, entity))
            } else {
                None
            }
        } else {
            println!("You don't have {thing}.");
            None
        }
    }

    pub fn get_inventory(&self) -> &HashSet<usize> {
        &self.inventory
    }

    pub fn get_entity(&self, entity_id: usize) -> Option<&Entity> {
        self.entities.get(&entity_id)
    }

    pub fn take_entity_from_room(&mut self, thing: &str) -> bool {
        let mut found_id = None;
        for id in self.rooms[self.loc].get_entities() {
            if let Some(entity) = self.entities.get(id) {
                if entity.aliases.contains(thing) {
                    found_id = Some(*id);
                }
            }
        }
        if let Some(id) = found_id {
            if self.rooms[self.loc].get_entity(id) {
                self.inventory.insert(id);
                return true;
            }
        }
        false
    }

    pub fn get_actor(&self, actor_id: usize) -> Option<&Actor> {
        self.actors.get(&actor_id)
    }

    pub fn special_event_triggered(&self, command: &Command) -> Option<Vec<Command>> {
        for event_id in self.active_events.iter() {
            let event = &self.events[*event_id];
            let condition = &self.conditions[event.condition_id];
            if self.check_condition(condition, command) {
                println!("{}", event.message);
                return Some(event.command_stack.clone());
            }
        }
        None
    }

    pub fn check_condition(&self, condition: &Condition, command: &Command) -> bool {
        match condition {
            Condition::Location(loc) => self.loc == *loc,
            Condition::And(c1, c2) => {
                self.check_condition(&self.conditions[*c1], command)
                    && self.check_condition(&self.conditions[*c2], command)
            }
            Condition::CommandIs(command_condition) => command_condition == command,
        }
    }

    pub fn set_file_name(&mut self, file_name: &str) {
        self.file_name = file_name.to_string();
    }

    pub fn get_file_name(&self) -> &str {
        &self.file_name
    }
    pub fn de_activate_event(&mut self, event_id: &usize) {
        
        self.active_events.remove(event_id);
    }
    pub fn activate_event(&mut self, event_id: &usize) {
        
        self.active_events.insert(*event_id);
    }
}
