use day4::cards;

fn main() {
    let input = include_str!("input.txt");

    println!("p1 = {:?}", p1(input));
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
}
