use crate::command::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Condition {
    CommandIs(Command),
    Location(usize),
    And(usize, usize),
    ObjectInInventory(usize),
    Or(usize, usize),
    NotCommandIs(Command),
    NotLocation(usize),
    NotAnd(usize, usize),
    NotObjectInInventory(usize),
    NotOr(usize, usize),
}
