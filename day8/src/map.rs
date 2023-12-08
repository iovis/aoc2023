use std::collections::HashMap;

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, multispace1, newline};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair, terminated};
use nom::IResult;

pub fn parse(input: &str) -> (&[u8], HashMap<&str, [&str; 2]>) {
    let (input, instructions) = parse_instructions(input).expect("instructions can be parsed");
    let (_, map) = parse_nodes(input).expect("nodes can be parsed");

    (instructions, map)
}

fn parse_instructions(input: &str) -> IResult<&str, &[u8]> {
    map(terminated(alpha1, multispace1), |instructions: &str| {
        instructions.as_bytes()
    })(input)
}

fn parse_nodes(input: &str) -> IResult<&str, HashMap<&str, [&str; 2]>> {
    map(separated_list1(newline, parse_node), |nodes| {
        nodes.into_iter().collect()
    })(input)
}

fn parse_node(input: &str) -> IResult<&str, (&str, [&str; 2])> {
    map(
        separated_pair(
            alpha1,
            tag(" = "),
            delimited(
                tag("("),
                separated_pair(alpha1, tag(", "), alpha1),
                tag(")"),
            ),
        ),
        |(key, (left, right))| (key, [left, right]),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_instructions_test() {
        let input = indoc::indoc! {"
            LLR

            AAA = (BBB, CCC)
        "};

        assert_eq!(
            parse_instructions(input),
            Ok(("AAA = (BBB, CCC)\n", "LLR".as_bytes()))
        );
    }

    #[test]
    fn parse_nodes_test() {
        let input = indoc::indoc! {"
            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
        "};

        assert_eq!(
            parse_nodes(input),
            Ok((
                "\n",
                HashMap::from([("AAA", ["BBB", "CCC"]), ("BBB", ["DDD", "EEE"]),])
            ))
        );
    }

    #[test]
    fn parse_map_test() {
        let input = indoc::indoc! {"
            LLR

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
        "};

        assert_eq!(
            parse(input),
            (
                "LLR".as_bytes(),
                HashMap::from([("AAA", ["BBB", "CCC"]), ("BBB", ["DDD", "EEE"]),])
            )
        );
    }
}
