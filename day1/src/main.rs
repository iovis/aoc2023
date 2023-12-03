use std::iter::IntoIterator;
mod parser;

fn main() {
    let input = include_str!("input.txt");

    let a1 = p1(input);
    println!("{a1:?}");

    let a2 = p2(input);
    println!("{a2:?}");
}

/// Find the first and last digit in a line
/// Combine them into a two digit number
/// Sum all the numbers
fn p1(input: &str) -> u64 {
    input
        .lines()
        .map(str::chars)
        .map(|chars| chars.filter(|c| c.is_numeric()))
        .map(build_number)
        .sum()
}

/// Now numbers can be spelled out ("one", "two", ...)
fn p2(input: &str) -> u64 {
    input
        .lines()
        .map(parser::parse_line)
        .map(IntoIterator::into_iter)
        .map(build_number)
        .sum()
}

/// Takes an Iterator of char and returns a number with the first and last digit
fn build_number<I>(mut chars: I) -> u64
where
    I: Iterator<Item = char>,
{
    let mut number: [u8; 2] = [0; 2];

    let first_digit = chars.next().expect("Always at least one number");
    number[0] = first_digit as u8;

    if let Some(second_digit) = chars.last() {
        number[1] = second_digit as u8;
    } else {
        number[1] = number[0];
    }

    std::str::from_utf8(&number)
        .expect("Should be a str")
        .parse()
        .expect("Should be a number")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let input = indoc::indoc! {"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "};

        assert_eq!(p1(input), 142);
    }

    #[test]
    fn p2_test() {
        let input = indoc::indoc! {"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "};

        assert_eq!(p2(input), 281);
    }
}
