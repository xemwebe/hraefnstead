use crate::direction::Direction;
//use std::io::{self, Write};

use crate::state::State;
use crate::victory::Victory;

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
    Won,
    Help(String),
    // Denial,
    //TriggerDialog,
    // StateOfDialog(usize),
}

impl Command {
    pub fn execute(&self, state: &mut State) -> Victory {
        match self {
            Command::Quit => {
                return Victory::Quit;
            }
            Command::Save(name) => return Victory::Save(name.clone()),
            //Save(name) => {
            //     let file_name = if name.is_empty() {
            //         state.get_file_name().to_string()
            //     } else {
            //         name.to_string()
            //     };
            //     state.set_file_name(&file_name);
            //     super::save_game(&file_name, state);
            Command::Load(name) => {
                return Victory::Load(name.clone());
                //let file_name = if name.is_empty() {
                //     state.get_file_name()
                //  } else {
                //      name
                //   };
                //   super::load_game(file_name);
                //  Command::Look.execute(state);
            }
            Command::Look => {
                let room = state.get_room();
                let mut msg = room.get_description().to_string();
                let exits = room.get_exits();
                if exits.is_empty() {
                    msg = format!("{msg}\nThere seems to be no exit.\n");
                } else {
                    msg = format!("{msg}\nExits:");
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
                    let msg = "\nYou can't go that way.".to_string();
                    state.log(&msg);
                }
            }
            Command::Take(thing) => {
                let msg = if state.take_entity_from_room(thing) {
                    "\nTaken.".to_string()
                } else {
                    format!("\nThere is no {thing} here.")
                };
                state.log(&msg);
            }
            Command::Drop(thing) => {
                let msg;
                if let Some((entity_id, entity)) = state.get_from_inventory(thing) {
                    msg = format!("\nYou drop the {}", entity.get_name());
                    let room = state.get_room_mut();
                    room.add_entity(entity_id);
                } else {
                    msg = format!("\nYou don't have a {thing} to drop.");
                }
                state.log(&msg);
            }
            Command::Inventory => {
                let mut msg;
                let inventory = state.get_inventory();
                if inventory.is_empty() {
                    msg = "\nYou are empty handed.".to_string();
                } else {
                    msg = "You have:".to_string();
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
                state.consume_from_inventory(id);
            }
            Command::Craft(thing) => {
                if let Some(id) = state.find_inventory(thing) {
                    if let Some(super_id) = state.get_craft_inventory().get(&id) {
                        state.why_not_mutable(*super_id);
                        Command::Eat(thing.to_string()).execute(state);
                        return Victory::Won;
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
                return Victory::GameOver;
            }
            Command::Won => return Victory::Won,
            Command::Help(command) => {
                let mut tokens = command.split_whitespace();
                let answer = tokens.next().unwrap();
                match answer{
                    "look"=>println!("With look you get a brief description of your surroundings"),
                    "save"=>println!("Saves your game for you"),
                    "load"=>println!("Loads a prior saved game file"),
                    "examine"=>println!("Gives you a detailed description of specified Item/Object. Can also be applied on items in your inventory"),
                    "inventory"=>println!("Shows all items you are currently carrying with you"),
                    "go"=>println!("With go you can navigate into any direction you specify(north/south/east/west)"),
                    "use"=>println!("With use you can perform specific actions that require a specific item. Make sure to specify said items when using 'use'"),
                    "attack"=>println!("Doesn't the name speak for itself? Just keep in mind messing with the wrong people WILL get you in trouble"),
                    "craft"=>println!("With craft you consume item(s) to create new ones, that are oftentimes from much higher quality and value than there components"),
                    "Default"=>println!("look\nquit\nsave\ngo\ndrop\ninventory\nexamine\nuse\nattack\ncraft"),
                    _=>{}

                }
            }

            _ => {}
        }

        Victory::None
    }
}
