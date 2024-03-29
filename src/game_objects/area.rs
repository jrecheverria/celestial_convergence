use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Passage {
    Blocked,
    Open,
}
// Represents an individual area a player can interact with
// It contains a target condition which if met, allows players to move to other areas
#[derive(Serialize, Deserialize, Debug)]
pub struct Area {
    name: String,
    description: String,
    target: String,
    state: Passage,
}