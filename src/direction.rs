use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "north" => Some(Direction::North),
            "south" => Some(Direction::South),
            "east" => Some(Direction::East),
            "west" => Some(Direction::West),
            _ => None,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::North => write!(f, "North"),
            Direction::South => write!(f, "South"),
            Direction::East => write!(f, "East"),
            Direction::West => write!(f, "West"),
        }
    }
}
