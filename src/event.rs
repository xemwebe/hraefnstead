use crate::command::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub condition_id: usize,
    pub message: String,
    pub command_stack: Vec<Command>,
}

impl Event {
    pub fn new(condition_id: usize, message: String, command_stack: Vec<Command>) -> Self {
        Self {
            condition_id,
            message,
            command_stack,
        }
    }
}
