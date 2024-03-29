use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
// Represents an individual area a player can interact with
// It contains a target condition which if met, allows players to move to other areas
pub struct Area {
    name: String,
    description: String
}