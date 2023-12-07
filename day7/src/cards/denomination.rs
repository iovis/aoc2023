use std::collections::HashMap;

use itertools::Itertools;

use super::Card;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Denomination {
    HighCard,
    Pair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<[Card; 5]> for Denomination {
    fn from(cards: [Card; 5]) -> Self {
        let mut map: HashMap<Card, u64> = HashMap::new();

        for card in cards {
            *map.entry(card).or_insert(0) += 1;
        }

        let jokers = map.remove(&Card::Joker).unwrap_or(0);

        // Sort values in descending order
        let repetitions = map.values().sorted().rev().collect_vec();

        // All jokers
        if repetitions.is_empty() {
            return Denomination::FiveOfAKind;
        }

        match repetitions[0] + jokers {
            5 => Denomination::FiveOfAKind,
            4 => Denomination::FourOfAKind,
            3 => {
                if *repetitions[1] == 2 {
                    Denomination::FullHouse
                } else {
                    Denomination::ThreeOfAKind
                }
            }
            2 => {
                if *repetitions[1] == 2 {
                    Denomination::TwoPairs
                } else {
                    Denomination::Pair
                }
            }
            1 => Denomination::HighCard,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_cards_test() {
        assert_eq!(
            Denomination::from([
                Card::Three,
                Card::Three,
                Card::Three,
                Card::Three,
                Card::Three
            ]),
            Denomination::FiveOfAKind
        );

        assert_eq!(
            Denomination::from([
                Card::Two,
                Card::Three,
                Card::Three,
                Card::Three,
                Card::Three
            ]),
            Denomination::FourOfAKind
        );

        assert_eq!(
            Denomination::from([Card::Two, Card::Two, Card::Three, Card::Three, Card::Three]),
            Denomination::FullHouse
        );

        assert_eq!(
            Denomination::from([Card::Two, Card::Ace, Card::Three, Card::Three, Card::Three]),
            Denomination::ThreeOfAKind
        );

        assert_eq!(
            Denomination::from([Card::Two, Card::Ace, Card::Two, Card::Three, Card::Three]),
            Denomination::TwoPairs
        );

        assert_eq!(
            Denomination::from([Card::Three, Card::Two, Card::Ten, Card::Three, Card::King]),
            Denomination::Pair
        );

        assert_eq!(
            Denomination::from([Card::Three, Card::Two, Card::Ten, Card::Queen, Card::King]),
            Denomination::HighCard
        );
    }

    #[test]
    fn from_cards_with_joker_test() {
        assert_eq!(
            Denomination::from([
                Card::Joker,
                Card::Three,
                Card::Three,
                Card::Three,
                Card::Three
            ]),
            Denomination::FiveOfAKind
        );

        assert_eq!(
            Denomination::from([
                Card::Joker,
                Card::Two,
                Card::Three,
                Card::Three,
                Card::Three
            ]),
            Denomination::FourOfAKind
        );

        assert_eq!(
            Denomination::from([
                Card::Joker,
                Card::Ace,
                Card::Joker,
                Card::Three,
                Card::Three
            ]),
            Denomination::FourOfAKind
        );

        assert_eq!(
            Denomination::from([Card::Joker, Card::Two, Card::Ten, Card::Queen, Card::King]),
            Denomination::Pair
        );
    }
}
