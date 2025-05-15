use clap::Parser;
use hraefnstead_lib::victory::Victory;
use hraefnstead_lib::{load_game, parser::parse, state::State, SAVE_FILE};
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

    let mut input = "look".to_string();

    let mut state = if cli.test {
        State::new()
    } else {
        let game_file = if let Some(file) = cli.game {
            file
        } else {
            SAVE_FILE.to_string()
        };
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
                            state = load_game(state.get_file_name());
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
