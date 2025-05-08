use crate::direction::Direction;

use crate::state::State;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum Command {
    Move(Direction),
    Look,
    Take(String),
    Drop(String),
    Examine(String),
    Save(String),
    Load(String),
    Quit,
    Inventory,
    AddItemToRoom(usize),
    None,
    DeActivateEvent(usize),
    ActivateEvent(usize),
    Use(String),
    RemoveActor(usize),
    //Empty,
    Eat(String),
    Consume(usize),
    AddExit(Direction, usize), // Denial,
                               //TriggerDialog,
                               // StateOfDialog(usize),
}

impl Command {
    pub fn execute(&self, state: &mut State) -> bool {
        match self {
            Command::Quit => {
                return false;
            }
            Command::Save(name) => {
                let file_name = if name.is_empty() {
                    state.get_file_name().to_string()
                } else {
                    name.to_string()
                };
                state.set_file_name(&file_name);
                super::save_game(&file_name, state);
            }
            Command::Load(name) => {
                let file_name = if name.is_empty() {
                    state.get_file_name()
                } else {
                    name
                };
                super::load_game(&file_name);
                Command::Look.execute(state);
            }
            Command::Look => {
                let room = state.get_room();
                println!("{}", room.get_description());
                let exits = room.get_exits();
                if exits.is_empty() {
                    println!("There seems to be no exit.")
                } else {
                    print!("Exits: ");
                    for (dir, _) in exits.iter() {
                        print!("{dir} ");
                    }
                    println!();
                }
                let actors = room.get_actors();
                if !actors.is_empty() {
                    for actor in actors.iter() {
                        if let Some(actor) = state.get_actor(*actor) {
                            println!("{}", actor.description);
                        }
                    }
                    println!();
                }
                let entities = room.get_entities();
                if entities.is_empty() {
                    println!("There is nothing here.")
                } else {
                    println!("You see:");
                    for e in entities.iter() {
                        if let Some(entity) = state.get_entity(*e) {
                            println!("{}", entity.get_name());
                        }
                    }
                }
            }
            Command::Move(dir) => {
                if let Some(new_room) = state.get_exit(dir.clone()) {
                    state.set_location(new_room);
                    Command::Look.execute(state);
                } else {
                    println!("You can't go that way.");
                }
            }
            Command::Take(thing) => {
                if state.take_entity_from_room(&thing) {
                    println!("Taken.");
                } else {
                    println!("There is no {} here.", thing);
                }
            }
            Command::Drop(thing) => {
                if let Some((entity_id, entity)) = state.get_from_inventory(&thing) {
                    println!("You drop the {}", entity.get_name());
                    let room = state.get_room_mut();
                    room.add_entity(entity_id);
                } else {
                    println!("You don't have a {} to drop.", thing);
                }
            }
            Command::Inventory => {
                let inventory = state.get_inventory();
                if inventory.is_empty() {
                    println!("You are empty handed.");
                } else {
                    println!("You have:");
                    for entity_id in inventory.iter() {
                        if let Some(entity) = state.get_entity(*entity_id) {
                            println!("{}", entity.name);
                        }
                    }
                }
            }
            Command::AddItemToRoom(entity_id) => {
                state.get_room_mut().add_entity(*entity_id);
            }
            Command::DeActivateEvent(event_id) => state.de_activate_event(event_id),
            Command::ActivateEvent(event_id) => state.activate_event(event_id),
            Command::Examine(thing) => {
                if let Some(id) = state.find_inventory(thing) {
                    if let Some(entity) = state.get_entity(id) {
                        println!("{}", entity.description);
                    }
                } else {
                    println!("You need to have item in inventory!")
                }
            }
            Command::Eat(thing) => {
                if let Some(id) = state.find_inventory(thing) {
                    state.consume_from_inventory(&id);
                } else {
                    println!("You need to have item in inventory!")
                }
            }
            Command::Consume(id) => {
                state.consume_from_inventory(&id);
            }
            Command::AddExit(direction, room_number) => state
                .get_room_mut()
                .add_exit(direction.clone(), *room_number),
            Command::RemoveActor(actor_id) => {
                state.get_room_mut().remove_actor(*actor_id);
            }
            _ => {}
        }
        true
    }
}
