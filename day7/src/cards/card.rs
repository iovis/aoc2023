#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Card {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<u8> for Card {
    fn from(value: u8) -> Self {
        match value {
            b'2' => Self::Two,
            b'3' => Self::Three,
            b'4' => Self::Four,
            b'5' => Self::Five,
            b'6' => Self::Six,
            b'7' => Self::Seven,
            b'8' => Self::Eight,
            b'9' => Self::Nine,
            b'T' => Self::Ten,
            b'J' => Self::Jack,
            b'Q' => Self::Queen,
            b'K' => Self::King,
            b'A' => Self::Ace,
            _ => unreachable!(),
        }
    }
}
