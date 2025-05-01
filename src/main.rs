mod action;
mod direction;
mod entity;
mod parser;
mod room;
mod state;

use action::Action;
use parser::parse;
use state::State;
use std::io::{self, Write};

const SAVE_FILE: &str = "adventure_state.json";

fn main() {
    let mut state = State::new();

    let mut input = "load".to_string();
    loop {
        match parse(&input) {
            Action::Quit => break,
            Action::Save => {
                let state_json = serde_json::to_string(&state).expect("Failed to serialize state");
                std::fs::write(SAVE_FILE, state_json).expect("Failed to write save file");
            }
            Action::Load => {
                let state_json =
                    std::fs::read_to_string(SAVE_FILE).expect("Failed to read save file");
                state = serde_json::from_str(&state_json).expect("Failed to deserialize state");
                input = "look".to_string();
                continue;
            }
            Action::Look => {
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
                let entities = room.get_entities();
                if entities.is_empty() {
                    println!("There is nothing here.")
                } else {
                    println!("You see:");
                    for e in entities.iter() {
                        println!("{}", e.1.get_description());
                    }
                }
            }
            Action::Move(dir) => {
                if let Some(new_room) = state.get_exit(dir) {
                    state.set_location(new_room);
                    input = "look".to_string();
                    continue;
                } else {
                    println!("You can't go that way.");
                }
            }
            Action::Take(thing) => {
                let room = state.get_room_mut();
                if let Some(entity) = room.get_entity(&thing) {
                    println!("You take the {}", entity.get_name());
                    state.add_to_inventory(entity);
                } else {
                    println!("There is no {} here.", thing);
                }
            }
            Action::Drop(thing) => {
                if let Some(entity) = state.get_from_inventory(&thing) {
                    println!("You drop the {}", entity.get_name());
                    let room = state.get_room_mut();
                    room.add_entity(entity);
                } else {
                    println!("You don't have a {} to drop.", thing);
                }
            }
            Action::Inventory => {
                let inventory = state.get_inventory();
                if inventory.is_empty() {
                    println!("You are empty handed.");
                } else {
                    println!("You have:");
                    for entity in inventory.values() {
                        println!("{}", entity.description);
                    }
                }
            }

            _ => println!("Time passes..."),
        }
        input = String::new();
        print!("\n---> ");
        io::stdout().flush().expect("Failed to flush");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input = input.to_lowercase();
    }
}
