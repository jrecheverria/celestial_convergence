use std::io::{stdin};

use celestial_convergence::game_manager::cli::Cli;


fn main() {
    let mut user_input = String::new();
    let cli = Cli {};
    

    loop {
        user_input.clear();
        stdin().read_line(&mut user_input).expect("Invalid input");
        cli.interpret_player_input(&user_input);
    }
}
