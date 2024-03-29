use crate::game_objects::{Civilization};

// This is responsible for interpreting user input and moving a player through the civilization
pub struct Cli<'a> {
    pub civilization: &'a Civilization,
}

impl<'a> Cli<'a> {
    // Interprets a user input, tokenizes, and provides the correct in-game output
    pub fn interpret_player_input(&self, input: &str) {
        let tokens: Vec<&str> = input.trim().split(' ').collect();
        
        //TODO: Introduce error handling if user input < 0
        let action = tokens[0];
        let direction = tokens[1];

        match action {
            "go"    =>  self.translate_player(&direction),
            "again" =>  println!("You used the again command."),
            "look"  =>  println!("You used the look command."),
            "open"  =>  println!("You used the open command"),
            "close" =>  println!("You used the close command"),
            _       =>  println!("I do not know the word {}", action)
        };
    }

    // Will tanslate a player to a designated direction
    fn translate_player(&self, direction: &str) {
        match direction {
            "north" =>  println!("You moved north."),
            "south" =>  println!("You moved south."),
            "east"  =>  println!("You moved east."),
            "west"  =>  println!("You moved west."),
            _       =>  println!("Invalid direction {}", direction)
        };
    }

    // Validates if the player can move in this direction
    fn validate_player_translation(&self, direction: &str) {

    }
}