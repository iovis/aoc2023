use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace1;
use nom::combinator::{all_consuming, map};
use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair, tuple};
use nom::IResult;

use super::{game, Cube, Game};

/// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
pub fn parse(input: &str) -> Game {
    #[rustfmt::skip]
    let (_rest, game) = all_consuming(
        map(
            tuple((
                parse_game_id,
                parse_sets,
            )),
            |(id, sets)| Game { id, sets },
        ),
    )(input)
    .expect("All games should be valid");

    game
}

fn parse_game_id(input: &str) -> IResult<&str, game::ID> {
    delimited(tag("Game "), nom::character::complete::u64, tag(": "))(input)
}

fn parse_sets(input: &str) -> IResult<&str, Vec<game::Set>> {
    separated_list1(tag("; "), parse_set)(input)
}

fn parse_set(input: &str) -> IResult<&str, game::Set> {
    map(separated_list1(tag(", "), parse_cubes), |turns| {
        let mut set: game::Set = HashMap::from([(Cube::Red, 0), (Cube::Green, 0), (Cube::Blue, 0)]);

        for (cube, count) in turns {
            set.insert(cube, count).expect("Cube is within the values");
        }

        set
    })(input)
}

fn parse_cubes(input: &str) -> IResult<&str, (Cube, u64)> {
    map(
        separated_pair(
            nom::character::complete::u64,
            multispace1,
            alt((tag("red"), tag("green"), tag("blue"))),
        ),
        |(number, cube)| (Cube::from(cube), number),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use Cube::*;

    #[test]
    fn parse_cubes_test() {
        assert_eq!(parse_cubes("3 blue"), Ok(("", (Blue, 3))));
        assert_eq!(parse_cubes("4 red"), Ok(("", (Red, 4))));
        assert_eq!(parse_cubes("2 green"), Ok(("", (Green, 2))));
    }

    #[test]
    fn parse_set_test() {
        assert_eq!(
            parse_set("3 blue, 4 red"),
            Ok(("", HashMap::from([(Red, 4), (Green, 0), (Blue, 3)])))
        );
        assert_eq!(
            parse_set("1 red, 2 green, 6 blue"),
            Ok(("", HashMap::from([(Red, 1), (Green, 2), (Blue, 6)])))
        );
        assert_eq!(
            parse_set("2 green"),
            Ok(("", HashMap::from([(Red, 0), (Green, 2), (Blue, 0)])))
        );
    }

    #[test]
    fn parse_sets_test() {
        assert_eq!(
            parse_sets("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Ok((
                "",
                vec![
                    HashMap::from([(Red, 4), (Green, 0), (Blue, 3)]),
                    HashMap::from([(Red, 1), (Green, 2), (Blue, 6)]),
                    HashMap::from([(Red, 0), (Green, 2), (Blue, 0)]),
                ]
            ))
        );
    }

    #[test]
    fn parse_game_id_test() {
        assert_eq!(
            parse_game_id("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Ok(("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 1))
        );
    }

    #[test]
    fn parse_test() {
        assert_eq!(
            parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Game {
                id: 1,
                sets: vec![
                    HashMap::from([(Red, 4), (Green, 0), (Blue, 3)]),
                    HashMap::from([(Red, 1), (Green, 2), (Blue, 6)]),
                    HashMap::from([(Red, 0), (Green, 2), (Blue, 0)]),
                ]
            }
        );
    }
}
