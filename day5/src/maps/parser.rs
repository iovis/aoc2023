use nom::bytes::complete::tag;
use nom::character::complete::{self, alpha1, multispace1, newline};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair, tuple};
use nom::IResult;

use crate::{Map, RangeMapping};

pub fn parse(input: &str) -> (Vec<u64>, Vec<Map>) {
    let (rest, seeds) = parse_seeds(input).expect("seeds should be parseable");
    let (_rest, maps) = parse_maps(rest).expect("maps should be parseable");

    (seeds, maps)
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    delimited(
        tag("seeds: "),
        separated_list1(multispace1, complete::u64),
        tag("\n\n"),
    )(input)
}

fn parse_maps(input: &str) -> IResult<&str, Vec<Map>> {
    separated_list1(tag("\n\n"), parse_map)(input)
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    map(
        tuple((
            separated_pair(alpha1, tag("-to-"), alpha1),
            tag(" map:\n"),
            parse_mappings,
        )),
        |((source, destination), _, mappings)| Map {
            source: source.into(),
            destination: destination.into(),
            mappings,
        },
    )(input)
}

fn parse_mappings(input: &str) -> IResult<&str, Vec<RangeMapping>> {
    separated_list1(newline, parse_mapping)(input)
}

fn parse_mapping(input: &str) -> IResult<&str, RangeMapping> {
    map(
        tuple((
            complete::u64,
            delimited(multispace1, complete::u64, multispace1),
            complete::u64,
        )),
        RangeMapping::from,
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_mapping_test() {
        let input = indoc::indoc! {"
            50 98 2
        "};

        assert_eq!(
            parse_mapping(input),
            Ok((
                "\n",
                RangeMapping {
                    source: 98,
                    destination: 50,
                    len: 2
                },
            ))
        );
    }

    #[test]
    fn parse_mappings_test() {
        let input = indoc::indoc! {"
            50 98 2
            52 50 48
        "};

        assert_eq!(
            parse_mappings(input),
            Ok((
                "\n",
                vec![
                    RangeMapping {
                        source: 98,
                        destination: 50,
                        len: 2
                    },
                    RangeMapping {
                        source: 50,
                        destination: 52,
                        len: 48
                    }
                ]
            ))
        );
    }

    #[test]
    fn parse_map_test() {
        let input = indoc::indoc! {"
            seed-to-soil map:
            50 98 2
            52 50 48
        "};

        assert_eq!(
            parse_map(input),
            Ok((
                "\n",
                Map {
                    source: "seed".to_string(),
                    destination: "soil".to_string(),
                    mappings: vec![
                        RangeMapping {
                            source: 98,
                            destination: 50,
                            len: 2
                        },
                        RangeMapping {
                            source: 50,
                            destination: 52,
                            len: 48
                        }
                    ]
                },
            ))
        );
    }

    #[test]
    fn parse_maps_test() {
        let input = indoc::indoc! {"
            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15
        "};

        assert_eq!(
            parse_maps(input),
            Ok((
                "\n",
                vec![
                    Map {
                        source: "seed".to_string(),
                        destination: "soil".to_string(),
                        mappings: vec![
                            RangeMapping {
                                source: 98,
                                destination: 50,
                                len: 2
                            },
                            RangeMapping {
                                source: 50,
                                destination: 52,
                                len: 48
                            }
                        ]
                    },
                    Map {
                        source: "soil".to_string(),
                        destination: "fertilizer".to_string(),
                        mappings: vec![
                            RangeMapping {
                                source: 15,
                                destination: 0,
                                len: 37
                            },
                            RangeMapping {
                                source: 52,
                                destination: 37,
                                len: 2
                            },
                            RangeMapping {
                                source: 0,
                                destination: 39,
                                len: 15
                            }
                        ]
                    }
                ]
            ))
        );
    }

    #[test]
    fn parse_seeds_test() {
        let input = indoc::indoc! {"
            seeds: 79 14 55 13

        "};

        assert_eq!(parse_seeds(input), Ok(("", vec![79, 14, 55, 13])));
    }

    #[test]
    fn parse_test() {
        let input = indoc::indoc! {"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15
        "};

        assert_eq!(
            parse(input),
            (
                vec![79, 14, 55, 13],
                vec![
                    Map {
                        source: "seed".to_string(),
                        destination: "soil".to_string(),
                        mappings: vec![
                            RangeMapping {
                                source: 98,
                                destination: 50,
                                len: 2
                            },
                            RangeMapping {
                                source: 50,
                                destination: 52,
                                len: 48
                            }
                        ]
                    },
                    Map {
                        source: "soil".to_string(),
                        destination: "fertilizer".to_string(),
                        mappings: vec![
                            RangeMapping {
                                source: 15,
                                destination: 0,
                                len: 37
                            },
                            RangeMapping {
                                source: 52,
                                destination: 37,
                                len: 2
                            },
                            RangeMapping {
                                source: 0,
                                destination: 39,
                                len: 15
                            }
                        ]
                    }
                ]
            )
        );
    }
}
