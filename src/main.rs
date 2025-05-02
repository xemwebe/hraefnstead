mod actor;
mod command;
mod condition;
mod direction;
mod entity;
mod event;
mod parser;
mod room;
mod state;

use clap::Parser;
use command::Command;
use parser::parse;
use state::State;
use std::io::{self, Write};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sets the game status file name, default is 'adventure_state.json'
    #[arg(short, long, value_name = "FILE")]
    game: Option<String>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    /// Test mode, suppress loading default game settings
    #[arg(short, long)]
    test: bool,
}

const SAVE_FILE: &str = "adventure_state.json";

fn load_game(name: &str) -> State {
    let state_json = std::fs::read_to_string(name).expect("Failed to read game file");
    serde_json::from_str(&state_json).expect("Failed to deserialize state")
}

fn save_game(name: &str, state: &State) {
    let state_json = serde_json::to_string(&state).expect("Failed to serialize state");
    std::fs::write(&name, state_json).expect("Failed to write game file");
}

fn execute_command(state: &mut State, command: &Command) -> bool {
    match command {
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
            save_game(&file_name, state);
        }
        Command::Load(name) => {
            let file_name = if name.is_empty() {
                state.get_file_name()
            } else {
                name
            };
            load_game(&file_name);
            execute_command(state, &Command::Look);
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
                execute_command(state, &Command::Look);
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
                        println!("{}", entity.description);
                    }
                }
            }
        }
        Command::AddItemToRoom(entity_id) => {
            state.get_room_mut().add_entity(*entity_id);
        }
        _ => {}
    }
    true
}

fn main() {
    let cli = Cli::parse();

    let mut input = "look".to_string();

    let mut state = if cli.test {
        State::new()
    } else {
        let game_file = if let Some(file) = cli.game {
            file
        } else {
            SAVE_FILE.to_string()
        };
        load_game(&game_file)
    };

    loop {
        let command = parse(&input);
        if let Some(command_stack) = state.special_event_triggered(&command) {
            for command in command_stack {
                execute_command(&mut state, &command);
            }
        }
        if !execute_command(&mut state, &command) {
            break;
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
