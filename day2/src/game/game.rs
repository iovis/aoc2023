use std::collections::HashMap;

use super::Cube;

pub type Set = HashMap<Cube, u64>;
pub type ID = u64;

#[derive(Debug, PartialEq, Eq)]
pub struct Game {
    pub id: ID,
    pub sets: Vec<Set>,
}

impl From<(ID, Vec<Set>)> for Game {
    fn from((id, sets): (ID, Vec<Set>)) -> Self {
        Game { id, sets }
    }
}
