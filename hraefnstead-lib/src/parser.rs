use crate::command::Command;
use crate::direction::Direction;

pub fn parse(input: &str) -> Command {
    let mut tokens = input.split_whitespace();
    let command = tokens.next().unwrap();

    match command {
        "look" => Command::Look,
        "quit" => {
            println!("Goodbye!");
            Command::Quit
        }
        "save" => {
            println!("Saving game...");
            let file_name = if let Some(file_name) = tokens.next() {
                file_name.to_string()
            } else {
                String::new()
            };
            Command::Save(file_name)
        }
        "load" => {
            println!("Loading game...");
            let file_name = if let Some(file_name) = tokens.next() {
                file_name.to_string()
            } else {
                String::new()
            };
            Command::Load(file_name)
        }
        "go" => {
            if let Some(dir) = tokens.next() {
                if let Some(direction) = Direction::from_str(dir) {
                    Command::Move(direction)
                } else {
                    println!("I don't know that direction.");
                    Command::None
                }
            } else {
                println!("You need to specify a direction to go to.");
                Command::None
            }
        }
        "north" | "n" => Command::Move(Direction::North),
        "south" | "s" => Command::Move(Direction::South),
        "east" | "e" => Command::Move(Direction::East),
        "west" | "w" => Command::Move(Direction::West),
        "take" | "t" => {
            if let Some(thing) = tokens.next() {
                Command::Take(thing.to_string())
            } else {
                println!("You need to specify an item to take.");
                Command::None
            }
        }
        "drop" => {
            if let Some(thing) = tokens.next() {
                Command::Drop(thing.to_string())
            } else {
                println!("You need to specify an item to drop.");
                Command::None
            }
        }
        "inventory" | "inv" | "i" => Command::Inventory,
        "examine" => {
            if let Some(thing) = tokens.next() {
                Command::Examine(thing.to_string())
            } else {
                println!("You need to specify an item to examine.");
                Command::None
            }
        }
        "use" => {
            if let Some(thing) = tokens.next() {
                Command::Use(thing.to_string())
            } else {
                println!("You need to specify an item to use.");
                Command::None
            }
        }
        "attack" => {
            if let Some(thing) = tokens.next() {
                Command::Attack(thing.to_string())
            } else {
                println!("You need to specify an enemy to atack.");
                Command::None
            }
        }
        "help" => {
            if let Some(thing) = tokens.next() {
                Command::Help(thing.to_string())
            } else {
                Command::Help("Default".to_string())
            }
        }

        "craft" => {
            if let Some(thing) = tokens.next() {
                thing.to_string();
                if thing.contains("help") {
                    Command::CraftHelp
                } else {
                    Command::Craft(thing.to_string())
                }
            } else {
                println!("You cant craft with that");
                Command::None
            }
        }
        _ => {
            println!("I don't understand that command.");
            Command::None
        }
    }
}
