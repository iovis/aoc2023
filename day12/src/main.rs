use itertools::{Itertools, MultiProduct};
use rayon::prelude::{FromParallelIterator, ParallelBridge, ParallelIterator};
use rayon::*;
use regex::bytes::{Match, Regex};

mod parser;

fn main() {
    let input = include_str!("input.txt");
    let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    println!("p1 = {:?}", p1(input));
    println!("p2 = {:?}", p2(input));
}

/// '.' -> operational
/// '#' -> damaged
/// '?' -> unknown
///
/// How many different possible arrangements per row?
/// Sum them
fn p1(input: &str) -> usize {
    let re = Regex::new(r"\?+").unwrap();
    let mut permutations = vec![];

    for n in 0..20 {
        permutations.push(itertools::repeat_n(b"#.".iter(), n).multi_cartesian_product());
    }

    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(symbols, results)| {
            let results = results
                .split(",")
                .map(|n| n.parse::<u64>().unwrap())
                .collect_vec();

            (symbols.as_bytes(), results)
        })
        .map(|(symbols, results)| {
            // eprintln!("symbols = {}", from_utf8(&symbols).unwrap());
            let matches = re.find_iter(symbols).collect_vec();

            (symbols, matches, results)
        })
        .map(|(mut symbols, mut matches, results)| {
            let mut valid = 0;
            let mut symbols = symbols.iter().copied().collect_vec();
            let mut valid = 0;

            build_string(
                &mut symbols,
                &mut matches,
                &permutations,
                &results,
                0,
                &mut valid,
            );

            valid
        })
        .sum()
}

fn p2(input: &str) -> usize {
    let re = Regex::new(r"\?+").unwrap();
    let mut permutations = vec![];

    for n in 0..99 {
        permutations.push(itertools::repeat_n(b"#.".iter(), n).multi_cartesian_product());
    }
    // let mut permutations = Vec::from_par_iter(
    //     (0..99)
    //         .par_bridge()
    //         .map(|i| itertools::repeat_n(b"#.".iter(), i).multi_cartesian_product()),
    // );

    input
        .lines()
        .par_bridge()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(symbols, results)| {
            let symbols = std::iter::repeat(symbols).take(5).join("?").into_bytes();
            let results = std::iter::repeat(results)
                .take(5)
                .join(",")
                .split(",")
                .map(|n| n.parse::<u64>().unwrap())
                .collect_vec();

            (symbols, results)
        })
        .map(|(mut symbols, results)| {
            let mut matches = re.find_iter(&symbols).collect_vec();
            let mut valid = 0;
            let mut symbols = symbols.iter().copied().collect_vec();
            let mut valid = 0;

            build_string(
                &mut symbols,
                &mut matches,
                &permutations,
                &results,
                0,
                &mut valid,
            );

            valid
        })
        .sum()
}

fn build_string(
    mut string: &mut [u8],
    matches: &mut Vec<Match>,
    permutations: &Vec<MultiProduct<std::slice::Iter<u8>>>,
    // permutations: &Vec<Vec<Vec<&u8>>>,
    results: &Vec<u64>,
    i: usize,
    mut valid: &mut usize,
) {
    let m = matches[i];
    let perms = permutations[m.len()].clone().collect_vec();

    for per in perms {
        for i in 0..per.len() {
            string[m.start() + i] = *per[i];
        }

        let j = i + 1;

        if j < matches.len() {
            build_string(string, matches, permutations, results, j, valid);
        } else {
            // eprintln!("string = {:?}", from_utf8(&string).unwrap());
            if parse_damaged(string) == *results {
                *valid += 1;
            }
        }
    }
}

fn parse_damaged(line: &[u8]) -> Vec<u64> {
    let mut current = 0;
    let mut result = vec![];

    for &c in line {
        if c == b'#' {
            current += 1;
        } else if current != 0 {
            result.push(current);
            current = 0;
        }
    }

    if current != 0 {
        result.push(current);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_damaged_test() {
        let input = b"#.#.###";

        assert_eq!(parse_damaged(input), vec![1, 1, 3]);
    }

    // #[test]
    // fn p1_test() {
    //     let input = indoc::indoc! {"
    //         .??..??...?##. 1,1,3
    //     "};
    //
    //     assert_eq!(p1(input), 21);
    // }

    #[test]
    fn p1_test() {
        let input = indoc::indoc! {"
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
        "};

        assert_eq!(p1(input), 21);
    }

    #[test]
    fn p2_test() {
        let input = indoc::indoc! {"
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
        "};

        assert_eq!(p2(input), 525152);
    }
}
