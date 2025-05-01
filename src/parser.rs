use crate::action::Action;
use crate::direction::Direction;

pub fn parse(input: &str) -> Action {
    let mut tokens = input.split_whitespace();
    let command = tokens.next().unwrap();
    match command {
        "look" => Action::Look,
        "quit" => {
            println!("Goodbye!");
            Action::Quit
        }
        "save" => {
            println!("Saving game...");
            Action::Save
        }
        "load" => {
            println!("Loading game...");
            Action::Load
        }
        "go" => {
            if let Some(dir) = tokens.next() {
                if let Some(direction) = Direction::from_str(dir) {
                    Action::Move(direction)
                } else {
                    println!("I don't know that direction.");
                    Action::None
                }
            } else {
                println!("You need to specify a direction to go to.");
                Action::None
            }
        }
        "north" | "n" => Action::Move(Direction::North),
        "south" | "s" => Action::Move(Direction::South),
        "east" | "e" => Action::Move(Direction::East),
        "west" | "w" => Action::Move(Direction::West),
        "take" | "t" => {
            if let Some(thing) = tokens.next() {
                Action::Take(thing.to_string())
            } else {
                println!("You need to specify an item to take.");
                Action::None
            }
        }
        "drop" => {
            if let Some(thing) = tokens.next() {
                Action::Drop(thing.to_string())
            } else {
                println!("You need to specify an item to drop.");
                Action::None
            }
        }
        "inventory" | "inv" | "i" => Action::Inventory,
        _ => {
            println!("I don't understand that command.");
            Action::None
        }
    }
}
