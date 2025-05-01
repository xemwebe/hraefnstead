use crate::direction::Direction;

pub enum Action {
    Move(Direction),
    Look,
    Take(String),
    Drop(String),
    Use(String),
    Save,
    Load,
    Quit,
    Inventory,
    None,
}
