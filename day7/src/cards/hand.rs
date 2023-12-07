use std::cmp::Ordering;

use super::{Card, Denomination};

#[derive(Debug, Clone, Copy)]
pub struct Hand {
    pub cards: [Card; 5],
    pub bid: u64,
    pub denomination: Denomination,
}

impl From<&str> for Hand {
    /// `32T3K 765`
    fn from(line: &str) -> Self {
        let (cards_buffer, bid) = line.split_once(' ').unwrap();

        let bid = bid.parse().unwrap();

        let mut card_bytes: [u8; 5] = [0; 5];
        card_bytes.copy_from_slice(cards_buffer.as_bytes());
        let cards: [Card; 5] = card_bytes.map(Card::from);

        Self::new(cards, bid)
    }
}

impl Hand {
    pub fn new(cards: [Card; 5], bid: u64) -> Self {
        Self {
            cards,
            bid,
            denomination: Denomination::from(cards),
        }
    }
}

/// Two hands are equal if all the cards are the same
impl Eq for Hand {}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

/// Ties decided by highest _first_ card, then second, etc
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.denomination.cmp(&other.denomination) {
            Ordering::Equal => {
                // If same hand, check each card
                for i in 0..self.cards.len() {
                    match self.cards[i].cmp(&other.cards[i]) {
                        Ordering::Equal => continue,
                        ord => return ord,
                    }
                }

                Ordering::Equal
            }
            ord => ord,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq_test() {
        assert_eq!(
            Hand {
                cards: [Card::Three, Card::Two, Card::Ten, Card::Three, Card::King],
                bid: 765,
                denomination: Denomination::Pair
            },
            Hand {
                cards: [Card::Three, Card::Two, Card::Ten, Card::Three, Card::King],
                bid: 765,
                denomination: Denomination::Pair
            }
        );
    }

    #[test]
    fn parse_hands_test() {
        assert_eq!(
            Hand::from("32T3K 765"),
            Hand::new(
                [Card::Three, Card::Two, Card::Ten, Card::Three, Card::King],
                765,
            )
        );
    }
}
