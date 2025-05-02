use crate::command::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Condition {
    CommandIs(Command),
    Location(usize),
    And(usize, usize),
}
