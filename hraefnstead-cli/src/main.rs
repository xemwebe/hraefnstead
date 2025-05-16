use clap::Parser;
use hraefnstead_lib::parser::parse;
use hraefnstead_lib::state::State;
use hraefnstead_lib::victory::Victory;
use std::io::{self, Write};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sets the game status file name, default is 'adventure_state.json'
    #[arg(short, long, value_name = "FILE")]
    game: Option<String>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    /// Test mode, suppress loading default game settings
    #[arg(short, long)]
    test: bool,
}

fn main() {
    let cli = Cli::parse();

    println!("Welcome to the dungeons of hraefnstead!\nType:'help' to briefly view possible actions.\nTyping said actions prior to 'help' will reveal more about their quality. ");

    let mut input = "look".to_string();
    let mut game_file = SAVE_FILE.to_string();

    let mut state = if cli.test {
        State::new()
    } else {
        if let Some(file) = cli.game {
            game_file = file;
        }
        load_game(&game_file)
    };
    let mut victory = Victory::None;
    loop {
        let command = parse(&input);
        if let Some(command_stack) = state.special_event_triggered(&command) {
            for command in command_stack {
                victory = command.execute(&mut state);
            }
        } else {
            victory = command.execute(&mut state);
        }

        match victory {
            Victory::GameOver => {
                loop {
                    let msg = "Would you like to try again? (yes/no): ".to_string();
                    state.log(&msg);
                    println!("{}", state.get_log());
                    let mut input = String::new();
                    io::stdout().flush().expect("Failed to flush");
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    input = input.to_lowercase();
                    let mut tokens = input.split_whitespace();
                    let answer = tokens.next().unwrap();
                    match answer {
                        "yes" => {
                            load_game(&game_file);
                            break;
                        }
                        "no" => return,
                        _ => {}
                    };
                }
                //state.log(&msg);
            }
            Victory::Won => {
                loop {
                    let msg = "!!!Congratulations You won the Game!!!\nWould you like to start a new Game? (yes/no): ".to_string();
                    state.log(&msg);
                    println!("{}", state.get_log());
                    let mut input = String::new();
                    io::stdout().flush().expect("Failed to flush");
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    input = input.to_lowercase();
                    let mut tokens = input.split_whitespace();
                    let answer = tokens.next().unwrap();
                    match answer {
                        "yes" => {
                            state = State::new();
                            break;
                        }
                        "no" => return,
                        _ => {}
                    };
                }
                //state.log(&msg);
            }
            Victory::Quit => return,
            Victory::None => {}
            Victory::Load(ref name) => {
                load_game(name);
            }
            Victory::Save(ref name) => {
                if !name.is_empty() {
                    game_file = name.to_string();
                }
                save_game(&game_file, &state);
            }
        }
        println!("{}", state.get_log());
        input = String::new();
        print!("\n---> ");
        io::stdout().flush().expect("Failed to flush");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input = input.to_lowercase();
        //parse(&input);
    }
}

pub const SAVE_FILE: &str = "adventure_state.json";

pub fn load_game(name: &str) -> State {
    let state_json = std::fs::read_to_string(name).expect("Failed to read game file");
    serde_json::from_str(&state_json).expect("Failed to deserialize state")
}
pub fn save_game(name: &str, state: &State) {
    let state_json = serde_json::to_string(&state).expect("Failed to serialize state");
    std::fs::write(name, state_json).expect("Failed to write game file");
}
