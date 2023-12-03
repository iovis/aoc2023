use std::ops::Range;

fn main() {
    let input = include_str!("input.txt");

    println!("p1 = {:?}", p1(input));
    println!("p2 = {:?}", p2(input));
}

/// If a symbol (not `.`) is found around a number cluster, it's active
/// Sum the active numbers
fn p1(input: &str) -> u64 {
    let engine: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let col_size = engine[0].len();
    let mut total = 0;

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
                    total += parse_number(&engine[i][number_range]);
                }
            }

            j += 1;
        }
    }

    total
}

struct Gear(u64, u64);

impl Gear {
    pub fn ratio(&self) -> u64 {
        self.0 * self.1
    }
}

/// A gear is a `*` adjacent to exactly 2 numbers
/// Multiply together the adjacent numbers of each gear
/// Sum all of those
fn p2(input: &str) -> u64 {
    let engine: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let col_size = engine[0].len();
    let mut total = 0;

    for i in 0..engine.len() {
        for j in 0..col_size {
            if engine[i][j] == b'*' {
                total += gear_from(&engine, (i, j)).map_or(0, |gear| gear.ratio());
            }
        }
    }

    total
}

fn gear_from(engine: &[&[u8]], location: (usize, usize)) -> Option<Gear> {
    let (gear_i, gear_j) = location;
    let col_max = engine[gear_i].len() - 1;

    let i_min = gear_i.saturating_sub(1);
    let i_max = std::cmp::min(gear_i + 1, engine.len() - 1);

    let j_min = gear_j.saturating_sub(1);
    let j_max = std::cmp::min(gear_j + 1, col_max);

    // for i in i_min..=i_max {
    //     eprintln!("{:?}", std::str::from_utf8(engine[i]).unwrap());
    // }

    let mut numbers: Vec<u64> = vec![];
    for i in i_min..=i_max {
        let mut j = j_min;
        // eprintln!("=> {:?}", std::str::from_utf8(engine[i]).unwrap());

        while j <= j_max {
            let mut number_start = None;
            let mut number_end = None;

            // eprintln!("j = [{j}] ({:?})", engine[i][j] as char);
            if !engine[i][j].is_ascii_digit() {
                j += 1;
                continue;
            }

            // Number found, start scanning
            number_start = Some(j);
            number_end = Some(j);

            // Scan left
            let mut k = j - 1;
            while engine[i][k].is_ascii_digit() {
                // eprintln!("<- [{k}] ({:?})", engine[i][k] as char);
                number_start = Some(k);

                if k == 0 {
                    break;
                }

                k -= 1;
            }

            // Scan right
            let mut k = j + 1;
            while k <= col_max && engine[i][k].is_ascii_digit() {
                // eprintln!("-> [{k}] ({:?})", engine[i][k] as char);
                number_end = Some(k);
                k += 1;
            }

            // If the range is fully defined, build number
            // eprintln!("(start, end) = ({number_start:?}, {number_end:?})");
            if let (Some(start), Some(end)) = (number_start, number_end) {
                let number_range = start..=end;
                let number = parse_number(&engine[i][number_range]);
                // eprintln!("number = {number:?}");
                numbers.push(number);
            }

            // Pointer is now where `k` stopper parsing
            j = k;
        }
    }

    // eprintln!("numbers = {numbers:?}");
    if numbers.len() == 2 {
        Some(Gear(numbers[0], numbers[1]))
    } else {
        None
    }
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

    #[test]
    fn p2_test() {
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

        assert_eq!(p2(input), 467_835);
    }
}
