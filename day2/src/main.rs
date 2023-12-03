use std::collections::HashMap;

use day2::game::{self, Cube, Maximum};

fn main() {
    let input = include_str!("input.txt");

    println!("p1 = {:?}", p1(input));
    println!("p2 = {:?}", p2(input));
}

/// Which games are possible with [12 Red, 13 Green, 14 Blue]?
/// Sum the Game ID of the possible ones
fn p1(input: &str) -> u64 {
    let limits = HashMap::from([(Cube::Red, 12), (Cube::Green, 13), (Cube::Blue, 14)]);

    input
        .lines()
        .map(game::parse)
        .filter_map(|game| {
            for set in game.sets {
                for (cube, count) in set {
                    if count > *limits.get(&cube).unwrap() {
                        return None;
                    }
                }
            }

            Some(game.id)
        })
        .sum()
}

/// What's the maximum number of each cube in each game?
/// Then multiply those together per game
/// Then sum all
fn p2(input: &str) -> u64 {
    input
        .lines()
        .map(game::parse)
        .map(Maximum::from)
        .map(Maximum::power)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let input = indoc::indoc! {"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "};

        assert_eq!(p1(input), 8);
    }

    #[test]
    fn p2_test() {
        let input = indoc::indoc! {"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "};

        assert_eq!(p2(input), 2286);
    }
}
