mod actor;
pub mod command;
mod condition;
mod direction;
mod entity;
mod event;
pub mod parser;
mod room;
pub mod state;
pub mod victory;

// We use u32 MAX to be safe on wasm32
pub const GAME_OVER: usize = u32::MAX as usize;
