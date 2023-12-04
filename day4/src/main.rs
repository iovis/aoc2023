use std::collections::HashMap;

use day4::cards::{self, Card};

fn main() {
    let input = include_str!("input.txt");

    println!("p1 = {:?}", p1(input));
    println!("p2 = {:?}", p2(input));
}

/// Each number you have is 1 point
/// Each time you score one point, it doubles the score of the whole card
/// Get total
///
/// Example:
/// - Card 1 = 4; score = 8
/// - Card 2 = 2; score = 2
/// - Card 3 = 2; score = 2
/// - Card 3 = 1; score = 1
fn p1(input: &str) -> u32 {
    input
        .lines()
        .map(cards::parse)
        .map(|card| card.score())
        .filter(|&score| score > 0)
        .map(|score| 2u32.pow(score - 1))
        .sum()
}

/// Now each time you score, you get the same number of copies of the next cards
///
/// Example:
/// - Card 10 = 5 matches => You get Card 11, 12, 13, 14, 15
/// - If I have Card 10 3 times => I get 3xCard 11, 3xCard 12, ...
///
/// Sum all the Cards you end up having (original + copies)
fn p2(input: &str) -> u32 {
    let original_cards: Vec<Card> = input.lines().map(cards::parse).collect();

    let mut deck: HashMap<cards::ID, u32> = original_cards
        .iter()
        .map(|card| card.id)
        .map(|id| (id, 1))
        .collect();

    for card in original_cards {
        let number_of_copies_of_current_card = *deck.get(&card.id).unwrap();

        (1..=card.score())
            // The cards I get are `card.id` + 1, 2, ..., card.score()
            .map(|n| card.id + n)
            // Increment counter for each of the new copies we get
            .for_each(|id| {
                deck.entry(id)
                    .and_modify(|counter| *counter += number_of_copies_of_current_card);
            });
    }

    deck.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let input = indoc::indoc! {"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "};

        assert_eq!(p1(input), 13);
    }

    #[test]
    fn p2_test() {
        let input = indoc::indoc! {"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "};

        assert_eq!(p2(input), 30);
    }
}
