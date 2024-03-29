use std::fs;
use serde::{Serialize, Deserialize};

use crate::game_objects::Area;

#[derive(Serialize, Deserialize, Debug)]
// Represents an entire civilization a user can explore and destroy/save
// A civilization is composed of multiple areas
pub struct Civilization {
    name: String,
    areas: Vec<Area>
}

impl Civilization {
    // Loads a new civilization from the RON file
    pub fn new() -> Self {
        let civilization_data = fs::read_to_string("src/game_objects/civilization_1.ron").expect("Could not open file");
        // println!("{civilization_data}");
        let civilization = ron::from_str(&civilization_data).expect("Could not load civilization from disk.");
        return civilization;
    }
}