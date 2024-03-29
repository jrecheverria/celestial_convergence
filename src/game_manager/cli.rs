// This is responsible for interpreting user input and moving a player through the civilization
pub struct Cli {}

impl Cli {
    // Interprets a user input, tokenizes, and provides the correct in-game output
    pub fn interpret_player_input(&self, input: &str) {
        let tokens: Vec<&str> = input.trim().split(' ').collect();
        let action = tokens[0];

        match action {
            "go"    =>  println!("You used the go command."),
            "again" =>  println!("You used the go command."),
            "look"  =>  println!("You used the look command."),
            "open"  =>  println!("You used the open command"),
            "close" =>  println!("You used the close command"),
            _       =>  println!("I do not know the word {}", action)
        };
    }

    pub fn translate_player(&self, direction: &str) {
        
    }
}