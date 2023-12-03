#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Cube {
    Red,
    Green,
    Blue,
}

impl From<&str> for Cube {
    fn from(value: &str) -> Self {
        match value {
            "red" => Self::Red,
            "green" => Self::Green,
            "blue" => Self::Blue,
            _ => unreachable!(),
        }
    }
}
