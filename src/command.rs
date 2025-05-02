use crate::direction::Direction;

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
}
