use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::combinator::map;
use nom::IResult;

// I don't know why I'm not capable of using a combinator to do this
pub fn parse_line(input: &str) -> Vec<char> {
    let mut rest = input;
    let mut numbers = vec![];

    while !rest.is_empty() {
        match parse_word_or_number(rest) {
            Ok((rest_result, number)) => {
                numbers.push(number);
                rest = rest_result;
            }
            Err(_) => rest = &rest[1..],
        }
    }

    numbers
}

fn parse_word_or_number(input: &str) -> IResult<&str, char> {
    alt((parse_word, parse_digit))(input)
}

fn parse_digit(input: &str) -> IResult<&str, char> {
    one_of("0123456789")(input)
}

fn parse_word(input: &str) -> IResult<&str, char> {
    let (_, number) = map(
        alt((
            tag("one"),
            tag("two"),
            tag("three"),
            tag("four"),
            tag("five"),
            tag("six"),
            tag("seven"),
            tag("eight"),
            tag("nine"),
        )),
        |number: &str| match number {
            "one" => '1',
            "two" => '2',
            "three" => '3',
            "four" => '4',
            "five" => '5',
            "six" => '6',
            "seven" => '7',
            "eight" => '8',
            "nine" => '9',
            _ => unreachable!(),
        },
    )(input)?;

    // Only advance one character at a time, apparently
    // numbers that overlap also count
    Ok((&input[1..], number))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_test() {
        assert_eq!(parse_line("one"), vec!['1']);
        assert_eq!(parse_line("one2three"), vec!['1', '2', '3']);
        assert_eq!(parse_line("abc1def2"), vec!['1', '2']);
        assert_eq!(parse_line("abctwodefthree"), vec!['2', '3']);
        assert_eq!(parse_line("abc1abc"), vec!['1']);
        assert_eq!(parse_line("abconeabc"), vec!['1']);
        assert_eq!(parse_line("eightwothree"), vec!['8', '2', '3']);
        assert_eq!(parse_line("abcone2threexyz"), vec!['1', '2', '3']);
        assert_eq!(parse_line("zoneight234"), vec!['1', '8', '2', '3', '4']);
        assert_eq!(
            parse_line("eightsevenvm6rpnine7twonex"),
            vec!['8', '7', '6', '9', '7', '2', '1']
        );
    }
}
