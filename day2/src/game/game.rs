use std::collections::HashMap;

use super::{Cube, Maximum};

pub type Set = HashMap<Cube, u64>;
pub type ID = u64;

#[derive(Debug, PartialEq, Eq)]
pub struct Game {
    pub id: ID,
    pub sets: Vec<Set>,
}

impl Game {
    pub fn is_possible(&self, limits: &Maximum) -> bool {
        for (cube, &count) in self.sets.iter().flatten() {
            match cube {
                Cube::Red => {
                    if count > limits.red {
                        return false;
                    }
                }
                Cube::Green => {
                    if count > limits.green {
                        return false;
                    }
                }
                Cube::Blue => {
                    if count > limits.blue {
                        return false;
                    }
                }
            }
        }

        true
    }
}

impl From<(ID, Vec<Set>)> for Game {
    fn from((id, sets): (ID, Vec<Set>)) -> Self {
        Game { id, sets }
    }
}
