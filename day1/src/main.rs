fn main() {
    let input = include_str!("input.txt");

    let a1 = p1(input);
    println!("{a1:?}");
}

/// Find the first and last digit in a line
/// Combine them into a two digit number
/// Sum all the numbers
fn p1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let mut numbers_iter = line.chars().filter(|c| c.is_numeric());

            // Always at least one digit
            let first_digit = numbers_iter.next().unwrap();
            let mut number = first_digit.to_string();

            if let Some(second_digit) = numbers_iter.last() {
                number.push(second_digit);
            } else {
                number.push(first_digit);
            }

            number.parse::<u64>().unwrap()
        })
        .sum()
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
}
