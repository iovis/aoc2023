use std::str::from_utf8;

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");

    println!("p1 = {:?}", p1(input));
    // println!("p2 = {:?}", p2(input));
}

// Find mirror in columns for odd groups (count columns to the left of the mirror)
// Find mirror in rows for even groups (count rows above the mirror, multiplied by 100)
fn p1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|pattern| {
            let pattern = pattern
                .lines()
                .map(|line| line.as_bytes().iter().copied().collect_vec())
                .collect_vec();

            let mut mirror = 0;
            'outer: for i in 0..pattern.len() - 1 {
                if pattern[i] != pattern[i + 1] {
                    continue;
                }

                let max = std::cmp::min(i, pattern.len() - i - 2);

                for ii in 1..=max {
                    if pattern[i - ii] != pattern[i + ii + 1] {
                        continue 'outer;
                    }
                }

                mirror = i + 1;
                break;
            }

            if mirror != 0 {
                return 100 * mirror;
            }

            'outer: for j in 0..pattern[0].len() - 1 {
                let mut equal = true;

                for i in 0..pattern.len() {
                    if pattern[i][j] != pattern[i][j + 1] {
                        equal = false;
                        break;
                    }
                }

                if equal {
                    let max = std::cmp::min(j, pattern[0].len() - j - 2);
                    for jj in 1..=max {
                        for ii in 0..pattern.len() {
                            if pattern[ii][j - jj] != pattern[ii][j + jj + 1] {
                                equal = false
                            }
                        }

                        if !equal {
                            continue 'outer;
                        }
                    }

                    mirror = j + 1;
                    break;
                }
            }

            mirror
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let input = indoc::indoc! {"
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.

            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        "};

        assert_eq!(p1(input), 405);
    }
}
