use std::collections::HashMap;

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

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Cube {
    Red,
    Green,
    Blue,
}

impl From<&str> for Cube {
    fn from(value: &str) -> Self {
        match value {
            "red" => Cube::Red,
            "green" => Cube::Green,
            "blue" => Cube::Blue,
            _ => unreachable!(),
        }
    }
}
