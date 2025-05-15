mod actor;
mod command;
mod condition;
mod direction;
mod entity;
mod event;
pub mod parser;
mod room;
pub mod state;
pub mod victory;

use state::State;

pub fn load_game(name: &str) -> State {
    let state_json = std::fs::read_to_string(name).expect("Failed to read game file");
    serde_json::from_str(&state_json).expect("Failed to deserialize state")
}

pub fn save_game(name: &str, state: &State) {
    let state_json = serde_json::to_string(&state).expect("Failed to serialize state");
    std::fs::write(name, state_json).expect("Failed to write game file");
}

pub const SAVE_FILE: &str = "adventure_state.json";
