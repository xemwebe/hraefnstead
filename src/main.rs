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
                command.execute(&mut state);
            }
        } else if !command.execute(&mut state) {
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
