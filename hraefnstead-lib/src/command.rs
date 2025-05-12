use crate::direction::Direction;
use std::io::{self, Write};

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
    Craft(String),
    CraftHelp,
    Attack(String),
    GameOver,
    // Denial,
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
                let mut msg = format!("{}", room.get_description());
                let exits = room.get_exits();
                if exits.is_empty() {
                    msg = format!("{msg}\nThere seems to be no exit.\n");
                } else {
                    msg = format!("{msg}Exits:");
                    for (dir, _) in exits.iter() {
                        msg = format!("{msg}{dir} ");
                    }
                    msg = format!("{msg}\n");
                }
                let actors = room.get_actors();
                if !actors.is_empty() {
                    for actor in actors.iter() {
                        if let Some(actor) = state.get_actor(*actor) {
                            msg = format!("\n{msg}{}", actor.description);
                        }
                    }
                    msg = format!("{msg}\n");
                }
                let entities = room.get_entities();
                if entities.is_empty() {
                    msg = format!("{msg}\nThere is nothing here.")
                } else {
                    msg = format!("{msg}\nYou see:");
                    for e in entities.iter() {
                        if let Some(entity) = state.get_entity(*e) {
                            msg = format!("{msg}\n{}", entity.get_name())
                        }
                    }
                }
                state.log(&msg);
            }
            Command::Move(dir) => {
                if let Some(new_room) = state.get_exit(dir.clone()) {
                    state.set_location(new_room);
                    Command::Look.execute(state);
                } else {
                    let msg = format!("\nYou can't go that way.");
                    state.log(&msg);
                }
            }
            Command::Take(thing) => {
                let mut msg = String::new();
                if state.take_entity_from_room(&thing) {
                    msg = format!("\nTaken.");
                } else {
                    msg = format!("\nThere is no {} here.", thing);
                }
                state.log(&msg);
            }
            Command::Drop(thing) => {
                let mut msg = String::new();
                if let Some((entity_id, entity)) = state.get_from_inventory(&thing) {
                    msg = format!("\nYou drop the {}", entity.get_name());
                    let room = state.get_room_mut();
                    room.add_entity(entity_id);
                } else {
                    msg = format!("\nYou don't have a {} to drop.", thing);
                }
                state.log(&msg);
            }
            Command::Inventory => {
                let mut msg = String::new();
                let inventory = state.get_inventory();
                if inventory.is_empty() {
                    msg = format!("\nYou are empty handed.");
                } else {
                    let mut msg = format! {"You have:"};
                    for entity_id in inventory.iter() {
                        if let Some(entity) = state.get_entity(*entity_id) {
                            msg = format!("{msg}\n{}", entity.name);
                        }
                    }
                }
                state.log(&msg)
            }
            Command::AddItemToRoom(entity_id) => {
                state.get_room_mut().add_entity(*entity_id);
            }
            Command::DeActivateEvent(event_id) => state.de_activate_event(event_id),
            Command::ActivateEvent(event_id) => state.activate_event(event_id),
            Command::Examine(thing) => {
                let mut msg = String::new();
                if let Some(id) = state.find_inventory(thing) {
                    if let Some(entity) = state.get_entity(id) {
                        msg = format!("{msg}\n{}", entity.description);
                    }
                } else {
                    msg = format!("{msg}\nYou need to have item in inventory!");
                }
                state.log(&msg)
            }
            Command::Eat(thing) => {
                let mut msg = String::new();
                if let Some(id) = state.find_inventory(thing) {
                    state.consume_from_inventory(&id);
                } else {
                    msg = format!("{msg}\nYou need to have item in inventory!")
                }
                state.log(&msg)
            }
            Command::Consume(id) => {
                state.consume_from_inventory(&id);
            }
            Command::Craft(thing) => {
                if let Some(id) = state.find_inventory(thing) {
                    if let Some(super_id) = state.get_craft_inventory().get(&id) {
                        state.why_not_mutable(*super_id);
                        Command::Eat(thing.to_string()).execute(state);
                    }
                }
            }
            Command::CraftHelp => state.craft_help(),
            Command::AddExit(direction, room_number) => state
                .get_room_mut()
                .add_exit(direction.clone(), *room_number),
            Command::RemoveActor(actor_id) => {
                state.get_room_mut().remove_actor(*actor_id);
            }
            Command::GameOver => {
                let mut cmd = Command::None;
                let mut msg = String::new();
                while cmd == Command::None {
                    msg = format!("Would you like to try again? (yes/no): ");
                    let mut input = String::new();
                    io::stdout().flush().expect("Failed to flush");
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    input = input.to_lowercase();
                    let mut tokens = input.split_whitespace();
                    let answer = tokens.next().unwrap();
                    cmd = match answer {
                        "yes" => Command::Load(state.get_file_name().to_string()),
                        "no" => Command::Quit,
                        _ => Command::None,
                    };
                }
                state.log(&msg);
                return cmd.execute(state);
            }

            _ => {}
        }

        true
    }
}
