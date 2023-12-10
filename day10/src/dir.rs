#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn iter() -> impl Iterator<Item = Direction> {
        use Direction::*;

        [North, East, South, West].into_iter()
    }
}

impl std::ops::Not for Direction {
    type Output = Direction;

    fn not(self) -> Self::Output {
        use Direction::*;

        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

pub trait DirectionExt {
    fn directions(self) -> Option<[Direction; 2]>;
    fn next_from(self, previous: Direction) -> Option<Direction>;
}

impl DirectionExt for u8 {
    /// | is a vertical pipe connecting north and south.
    /// - is a horizontal pipe connecting east and west.
    /// L is a 90-degree bend connecting north and east.
    /// J is a 90-degree bend connecting north and west.
    /// 7 is a 90-degree bend connecting south and west.
    /// F is a 90-degree bend connecting south and east.
    /// . is ground; there is no pipe in this tile.
    fn directions(self) -> Option<[Direction; 2]> {
        use Direction::*;

        match self {
            b'|' => Some([North, South]),
            b'-' => Some([West, East]),
            b'L' => Some([North, East]),
            b'J' => Some([North, West]),
            b'7' => Some([West, South]),
            b'F' => Some([East, South]),
            _ => {
                // eprintln!("symbol = {}", self as char);
                None
            }
        }
    }

    // -> West
    //   [7] directions: [West, South]
    //   => South
    fn next_from(self, from: Direction) -> Option<Direction> {
        let directions = self.directions()?;

        match directions {
            [prev, next] if prev == from => Some(next),
            [next, prev] if prev == from => Some(next),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Direction::*;

    #[test]
    fn directions_from_char_test() {
        assert_eq!(b'|'.directions(), Some([North, South]));
    }

    #[test]
    fn negation_test() {
        assert_eq!(!North, South);
    }

    #[test]
    fn next_direction_test() {
        assert_eq!(b'|'.next_from(North), Some(South));
        assert_eq!(b'|'.next_from(East), None);
        assert_eq!(b'|'.next_from(South), Some(North));
        assert_eq!(b'|'.next_from(West), None);

        assert_eq!(b'7'.next_from(North), None);
        assert_eq!(b'7'.next_from(East), None);
        assert_eq!(b'7'.next_from(South), Some(West));
        assert_eq!(b'7'.next_from(West), Some(South));
    }
}
