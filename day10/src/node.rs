#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Path,
    Out,
    In,
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub struct Node {
    pub symbol: u8,
    pub state: State,
    pub visited: bool,
}

impl Node {
    pub fn new(symbol: u8) -> Self {
        Self {
            symbol,
            state: State::Unknown,
            visited: false,
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self.state {
            State::Path => match self.symbol {
                b'|' => "│",
                b'-' => "─",
                b'L' => "╰",
                b'J' => "╯",
                b'7' => "╮",
                b'F' => "╭",
                b'S' => "S",
                _ => "E", // Point shouldn't be in path
            },
            State::Out => "×",
            State::In => "o",
            State::Unknown => "?",
        };

        write!(f, "{char}")
    }
}
