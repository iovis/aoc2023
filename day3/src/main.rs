use std::ops::Range;

fn main() {
    let input = include_str!("input.txt");

    println!("p1 = {:?}", p1(input));
}

/// If a symbol (not `.`) is found around a number cluster, it's active
/// Sum the active numbers
fn p1(input: &str) -> u64 {
    let engine: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let col_size = engine[0].len();
    let mut sum = 0;

    for i in 0..engine.len() {
        let mut j = 0;
        let mut number_start = None;
        let mut number_end = None;

        while j < col_size {
            if engine[i][j].is_ascii_digit() {
                // Start counting number if we didn't have one
                if number_start.is_none() {
                    number_start = Some(j);
                }

                // If row ended, finish number
                if j == col_size - 1 {
                    number_end = Some(j + 1);
                }
            } else if number_start.is_some() {
                number_end = Some(j);
            }

            // If the range is fully defined, check for parts
            if let (Some(start), Some(end)) = (number_start, number_end) {
                let number_range = start..end;
                number_start = None;
                number_end = None;

                if is_part(&engine, i, &number_range) {
                    sum += parse_number(&engine[i][number_range]);
                }
            }

            j += 1;
        }
    }

    sum
}

fn parse_number(digits: &[u8]) -> u64 {
    std::str::from_utf8(digits)
        .expect("is ascii number")
        .parse()
        .expect("is u64")
}

fn is_part(engine: &[&[u8]], row: usize, range: &Range<usize>) -> bool {
    let i_min = row.saturating_sub(1);
    let j_min = range.start.saturating_sub(1);

    let j_max = std::cmp::min(range.end, engine[row].len() - 1);
    let i_max = std::cmp::min(row + 1, engine.len() - 1);

    // for i in i_min..=i_max {
    //     eprintln!(
    //         "{:?}",
    //         std::str::from_utf8(&engine[i][j_min..=j_max]).unwrap()
    //     );
    // }

    for i in i_min..=i_max {
        for j in j_min..=j_max {
            if is_symbol(engine[i][j]) {
                return true;
            }
        }
    }

    false
}

fn is_symbol(byte: u8) -> bool {
    !byte.is_ascii_digit() && byte != b'.'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let input = indoc::indoc! {"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+..58
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "};

        assert_eq!(p1(input), 4361);
    }
}
