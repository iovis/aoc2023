use nom::bytes::complete::tag;
use nom::character::complete::multispace1;
use nom::combinator::{all_consuming, map};
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, separated_pair, tuple};
use nom::IResult;

use super::{card, Card};

pub fn parse(input: &str) -> Card {
    let (_rest, card) = all_consuming(map(
        pair(
            parse_card_id,
            separated_pair(
                parse_numbers,
                tuple((multispace1, tag("|"), multispace1)),
                parse_numbers,
            ),
        ),
        |(id, (winning_numbers, played_numbers))| Card {
            id,
            winning_numbers,
            played_numbers,
        },
    ))(input)
    .expect("Card should be parseable");

    card
}

fn parse_card_id(input: &str) -> IResult<&str, card::ID> {
    delimited(
        pair(tag("Card"), multispace1),
        nom::character::complete::u32,
        pair(tag(":"), multispace1),
    )(input)
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(multispace1, nom::character::complete::u32)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_numbers_test() {
        assert_eq!(
            parse_numbers("41 48 83 86 17"),
            Ok(("", vec![41, 48, 83, 86, 17]))
        );
    }

    #[test]
    fn parse_card_id_test() {
        assert_eq!(
            parse_card_id("Card   1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            Ok(("41 48 83 86 17 | 83 86  6 31 17  9 48 53", 1))
        );
    }

    #[test]
    fn parse_test() {
        assert_eq!(
            parse("Card 1: 41 48 83 86 17 |  83 86  6 31 17  9 48 53"),
            Card {
                id: 1,
                winning_numbers: vec![41, 48, 83, 86, 17],
                played_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
            }
        );
    }
}
