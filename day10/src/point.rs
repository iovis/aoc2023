use crate::dir::Direction;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point(pub usize, pub usize);

impl From<(usize, usize)> for Point {
    fn from((i, j): (usize, usize)) -> Self {
        Self(i, j)
    }
}

impl Point {
    pub fn coords(&self) -> (usize, usize) {
        (self.0, self.1)
    }

    pub fn symbol(&self, map: &[&[u8]]) -> u8 {
        map[self.0][self.1]
    }

    pub fn go(&self, direction: Direction, (max_row, max_col): (usize, usize)) -> Option<Self> {
        match direction {
            Direction::North => {
                let new = self.0.checked_sub(1)?;

                Some(Self(new, self.1))
            }
            Direction::East => {
                let new = self.1 + 1;
                if new > max_col {
                    return None;
                }

                Some(Self(self.0, new))
            }
            Direction::South => {
                let new = self.0 + 1;
                if new > max_row {
                    return None;
                }

                Some(Self(new, self.1))
            }
            Direction::West => {
                let new = self.1.checked_sub(1)?;

                Some(Self(self.0, new))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;
    use crate::dir::Direction::*;

    #[test]
    fn go_test() {
        let point = Point(0, 0);
        let limits = (5, 5);

        assert_eq!(point.go(North, limits), None);
        assert_eq!(point.go(East, limits), Some(Point(0, 1)));
        assert_eq!(point.go(South, limits), Some(Point(1, 0)));
        assert_eq!(point.go(West, limits), None);

        let point = Point(5, 5);
        let limits = (5, 5);

        assert_eq!(point.go(North, limits), Some(Point(4, 5)));
        assert_eq!(point.go(East, limits), None);
        assert_eq!(point.go(South, limits), None);
        assert_eq!(point.go(West, limits), Some(Point(5, 4)));

        let point = Point(2, 2);
        let all_directions = Direction::iter()
            .filter_map(|dir| point.go(dir, limits))
            .collect_vec();

        assert_eq!(
            all_directions,
            vec![Point(1, 2), Point(2, 3), Point(3, 2), Point(2, 1)]
        );
    }
}
