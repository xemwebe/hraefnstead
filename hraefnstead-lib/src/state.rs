use crate::actor::Actor;
use crate::command::Command;
use crate::condition::Condition;
use crate::direction::Direction;
use crate::entity::Entity;
//use crate::event::Dialog;
use crate::event::Event;
use crate::room::Room;

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    loc: usize,
    inventory: HashSet<usize>,
    craft_inventory: HashMap<usize, usize>,
    rooms: Vec<Room>,
    entities: HashMap<usize, Entity>,
    actors: HashMap<usize, Actor>,
    active_events: HashSet<usize>,
    events: Vec<Event>,
    //dialogs: Vec<Dialog>,
    conditions: Vec<Condition>,
    file_name: String,
    log: String,
}
impl Default for State {
    fn default() -> Self {
        Self::new()
    }
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

        let mut bag_of_chips_aliases = HashSet::new();
        bag_of_chips_aliases.insert("chips".to_string());
        let bag_of_chips = Entity::new(
            "Bag of chips",
            "The chips don't really look that bad, the smell however, suggests otherwise",
            bag_of_chips_aliases,
        );

        let mut golden_armor_aliases = HashSet::new();
        golden_armor_aliases.insert("armor".to_string());
        let golden_armor = Entity::new(
            "armor",
            "A really shiny, yet very powerful piece of armor",
            golden_armor_aliases,
        );
        // golden_armor_aliases.insert("golden armor".to_string());
        let mut dead_goblin_aliases = HashSet::new();
        dead_goblin_aliases.insert("goblin".to_string());
        dead_goblin_aliases.insert("corpse".to_string());
        let dead_goblin = Entity::new(
            "Goblin corpse",
            "The corpse smells badly and is rotting slowly.",
            dead_goblin_aliases,
        );

        let mut entity_map = HashMap::new();
        entity_map.insert(1, treasure);
        entity_map.insert(2, coin);
        entity_map.insert(3, vending_machine);
        entity_map.insert(4, bag_of_chips);
        entity_map.insert(5, golden_armor);
        entity_map.insert(6, dead_goblin);

        let mut actors_map = HashMap::new();
        let mut goblin_aliases = HashSet::new();
        goblin_aliases.insert("goblin".to_string());
        actors_map.insert(
            1,
            Actor::new(
                "Goblin",
                "A small red goblin leans against a door to the north.",
                goblin_aliases,
            ),
        );

        let mut chamber_actors = HashSet::new();
        chamber_actors.insert(1);

        let mut treasure_entities = HashSet::new();
        treasure_entities.insert(1);

        let mut entrance_entities = HashSet::new();
        entrance_entities.insert(3);

        let mut craft_inventory: HashMap<usize, usize> = HashMap::new();
        craft_inventory.insert(1, 5);

        let conditions = vec![
            Condition::Location(2),
            Condition::CommandIs(Command::Examine("bed".to_string())),
            Condition::And(0, 1),
            Condition::ObjectInInventory(2),
            Condition::Location(0),
            Condition::CommandIs(Command::Use("coin".to_string())),
            Condition::And(3, 4),
            Condition::And(5, 6),
            Condition::ObjectInInventory(4),
            Condition::CommandIs(Command::Use("goblin".to_string())),
            Condition::And(9, 8),
            Condition::And(10, 0),
            Condition::CommandIs(Command::Attack("goblin".to_string())),
            Condition::Actor(1),
            Condition::And(12, 13),
            Condition::And(14, 0),
            Condition::ObjectInInventory(5),
            Condition::CommandIs(Command::Craft("gold".to_string())),
            Condition::And(16, 17),
        ];

        let events = vec![
            Event::new(2,
                "The bed is made of soft wood and has a comfortable mattress. Below the pillow you find a copper coin".to_string(),
                vec![Command::AddItemToRoom(2), Command::DeActivateEvent(0), Command::ActivateEvent(1)]),
            Event::new(2,
                "Now that you have taken the coin, you glance down at an empty bed".to_string(),
                vec![]),
            Event::new(7,
            "The vending machine makes some concerning noice... but it works!".to_string(),
            vec![Command::DeActivateEvent(2), Command::ActivateEvent(3), Command::AddItemToRoom(4), Command::Consume(2), Command::ActivateEvent(4)]),
            Event::new(7,
                "You would sure like to get more loot, however your only coin is now gone".to_string(),
                vec![]),
            Event::new(11,
                "The goblin doesn't seem to take much interest in you, but he hungrily takes the chips.\nThe goblins face turns green, than grey.\nHe falls to the floow and doesn't move anymore.".to_string(),
                vec![Command::AddExit(Direction::North, 3), Command::RemoveActor(1), Command::AddItemToRoom(6)]),
            Event::new(15,
                "The Goblin's Fist hits (you) like a truck and lands you on the ground, where you get knocked out".to_string(),
                vec![Command::GameOver]),
            // Event::new(18,
            //     "!!!Congratulations you crafted the golden armor and won the game!!!".to_string(),
            //     vec![Command::Won] ),
        ];

        //let dialogs = vec![];

        let mut active_events = HashSet::new();
        active_events.insert(0);
        active_events.insert(2);
        active_events.insert(5);
        // active_events.insert(6);

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
            craft_inventory,
            entities: entity_map,
            actors: actors_map,
            active_events,
            events,
            //dialogs,
            conditions,
            log: String::new(),
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
            None
        }
    }

    pub fn get_inventory(&self) -> &HashSet<usize> {
        &self.inventory
    }

    pub fn get_craft_inventory(&self) -> &HashMap<usize, usize> {
        &self.craft_inventory
    }

    pub fn craft_help(&mut self) {
        let mut msg = String::new();
        for e in &self.inventory {
            if self.craft_inventory.contains_key(e) {
                if let Some(entity) = self.get_entity(*e) {
                    msg = format!("{} ---> ", entity.name);
                }
                if let Some(f) = self.craft_inventory.get(e) {
                    if let Some(entity) = self.get_entity(*f) {
                        msg = format!("{msg}{}", entity.name);
                    }
                }
            }
        }
        self.log(&msg);
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
            if self.rooms[self.loc].remove_entity(id) {
                self.inventory.insert(id);
                return true;
            }
        }
        false
    }

    pub fn get_actor(&self, actor_id: usize) -> Option<&Actor> {
        self.actors.get(&actor_id)
    }

    pub fn special_event_triggered(&mut self, command: &Command) -> Option<Vec<Command>> {
        let mut msg = String::new();
        let mut command_stack = None;
        for event_id in self.active_events.iter() {
            let event = &self.events[*event_id];
            let condition = &self.conditions[event.condition_id];
            if self.check_condition(condition, command) {
                msg = format!("{}\n", event.message);
                command_stack = Some(event.command_stack.clone());
                break;
            }
        }
        self.log(&msg);
        command_stack
    }

    pub fn check_condition(&self, condition: &Condition, command: &Command) -> bool {
        match condition {
            Condition::Location(loc) => self.loc == *loc,
            Condition::And(c1, c2) => {
                self.check_condition(&self.conditions[*c1], command)
                    && self.check_condition(&self.conditions[*c2], command)
            }
            Condition::Actor(actor_id) => self.actors.contains_key(actor_id),
            Condition::CommandIs(command_condition) => command_condition == command,
            Condition::ObjectInInventory(entity_id) => self.inventory.contains(entity_id),
            Condition::Or(c1, c2) => {
                self.check_condition(&self.conditions[*c1], command)
                    || self.check_condition(&self.conditions[*c2], command)
            }
            Condition::NotLocation(loc) => self.loc != *loc,
            Condition::NotOr(c1, c2) => {
                !self.check_condition(&self.conditions[*c1], command)
                    && !self.check_condition(&self.conditions[*c2], command)
            }
            Condition::NotCommandIs(command_condition) => command_condition != command,
            Condition::NotObjectInInventory(entity_id) => !self.inventory.contains(entity_id),
            Condition::NotAnd(c1, c2) => {
                !self.check_condition(&self.conditions[*c1], command)
                    || !self.check_condition(&self.conditions[*c2], command)
            }
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
    pub fn find_inventory(&mut self, thing: &str) -> Option<usize> {
        for id in self.inventory.iter() {
            if let Some(entity) = self.entities.get(id) {
                if entity.aliases.contains(thing) {
                    return Some(*id);
                }
            }
        }
        None
    }
    pub fn consume_from_inventory(&mut self, id: &usize) {
        self.inventory.remove(id);
        let mut msg = String::new();
        let removed = self.entities.get(id);
        if let Some(removed) = removed {
            msg = format!("{msg}\nConsumed {}", removed.get_name());
            println!("Consumed {}", removed.get_name());
        }
        self.log(&msg);
    }
    pub fn why_not_mutable(&mut self, mega_id: usize) {
        self.inventory.insert(mega_id);
    }

    pub fn log(&mut self, msg: &str) {
        self.log = format!("{}\n{msg}", self.log);
    }

    pub fn get_log(&mut self) -> String {
        let log = self.log.clone();
        self.log = String::new();
        log
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::State;
    use crate::victory::Victory;

    #[test]
    pub fn it_works() {
        let mut state = State::new();
        state.inventory.insert(1);
        Command::Craft("gold".to_string()).execute(&mut state);
        assert!(state.inventory.contains(&5) && !state.inventory.contains(&1))
    }

    #[test]
    fn does_it_work() {
        let mut state = State::new();
        state.inventory.insert(2);
        assert_eq!(Command::Inventory.execute(&mut state), Victory::None);
    }
}
