use clap::Parser;
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

    loop {
        let command = parse(&input);
        if let Some(command_stack) = state.special_event_triggered(&command) {
            for command in command_stack {
                if !command.execute(&mut state) {
                    return;
                }
            }
        } else if !command.execute(&mut state) {
            break;
        }
        println!("{}", state.get_log());
        input = String::new();
        print!("\n---> ");
        io::stdout().flush().expect("Failed to flush");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input = input.to_lowercase();
    }
}
