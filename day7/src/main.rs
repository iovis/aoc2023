use day7::cards::Hand;
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");

    // println!("p1 = {:?}", p1(input));
    println!("p2 = {:?}", p2(input));
}

/// Sort all the hands played by order of winner
/// multiply bid by ranking
/// sum
fn p1(input: &str) -> u64 {
    input
        .lines()
        .map(Hand::from)
        .sorted()
        .enumerate()
        .map(|(rank, hand)| hand.bid * (1 + rank as u64))
        .sum()
}

/// Now `J` is the Jocker
/// In repeats, acts like whatever makes the hand stronger
/// In tie breakers, they're the weakest card
///
/// multiply bid by ranking
/// sum
fn p2(input: &str) -> u64 {
    input
        .lines()
        .map(Hand::from)
        .sorted()
        .enumerate()
        .map(|(rank, hand)| hand.bid * (1 + rank as u64))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn p1_test() {
    //     let input = indoc::indoc! {"
    //         32T3K 765
    //         T55J5 684
    //         KK677 28
    //         KTJJT 220
    //         QQQJA 483
    //     "};
    //
    //     assert_eq!(p1(input), 6440);
    // }

    #[test]
    fn p2_test() {
        let input = indoc::indoc! {"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "};

        assert_eq!(p2(input), 5905);
    }
}
