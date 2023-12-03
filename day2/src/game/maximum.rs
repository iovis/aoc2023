use super::{Cube, Game};

#[derive(Debug, Default)]
pub struct Maximum {
    pub red: u64,
    pub green: u64,
    pub blue: u64,
}

impl Maximum {
    pub fn power(self) -> u64 {
        self.red * self.green * self.blue
    }
}

impl From<Game> for Maximum {
    fn from(game: Game) -> Self {
        let mut maximum = Self::default();

        for (cube, &count) in game.sets.iter().flatten() {
            match cube {
                Cube::Red => {
                    if count > maximum.red {
                        maximum.red = count;
                    }
                }
                Cube::Green => {
                    if count > maximum.green {
                        maximum.green = count;
                    }
                }
                Cube::Blue => {
                    if count > maximum.blue {
                        maximum.blue = count;
                    }
                }
            }
        }

        maximum
    }
}
