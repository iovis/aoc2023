use std::vec;

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");

    println!("p1 = {:?}", p1(input));
    println!("p2 = {:?}", p2(input, 1000000));
}

/// `.` is empty space, `#` are galaxies
/// Any row/col without galaxies needs to be duplicated
/// Sum all the shortest paths between galaxies
/// => For 9 galaxies: N*(N-1)/2 = 36 pairs
fn p1(input: &str) -> i64 {
    let mut map = input
        .lines()
        .map(|line| line.as_bytes().iter().copied().collect_vec())
        .collect_vec();

    let mut empty_row_ids = vec![];
    let mut empty_col_ids = vec![];

    // Check for empty rows/cols to duplicate
    for (i, row) in map.iter().enumerate() {
        if row.iter().all(|&c| c == b'.') {
            empty_row_ids.push(i);
        }
    }

    for j in 0..map[0].len() {
        let mut empty = true;

        for i in 0..map.len() {
            if map[i][j] == b'#' {
                empty = false;
                break;
            }
        }

        if empty {
            empty_col_ids.push(j);
        }
    }

    // print(&map);

    // Build the columns
    for row in &mut map {
        for &j in empty_col_ids.iter().rev() {
            row.insert(j, b'.');
        }
    }

    for &i in empty_row_ids.iter().rev() {
        let empty_row = map[i].clone();
        map.insert(i, empty_row);
    }

    // print(&map);

    let mut galaxies = vec![];

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == b'#' {
                galaxies.push((i as i64, j as i64));
            }
        }
    }

    // Get unique pairs of galaxies
    let pairs = galaxies.iter().combinations(2).collect_vec();

    // (6, 1)
    // (11, 5)
    // 6 - 11 = 5
    // 5 - 1 = 4
    // => 9
    pairs
        .iter()
        .map(|galaxies| (galaxies[0], galaxies[1]))
        .map(|(a, b)| {
            let x = (a.0 - b.0).abs();
            let y = (a.1 - b.1).abs();

            x + y
        })
        .sum()
}

/// Now it expanded one million times, sigh
fn p2(input: &str, expansion: usize) -> i64 {
    let mut map = input
        .lines()
        .map(|line| line.as_bytes().iter().copied().collect_vec())
        .collect_vec();

    let mut empty_row_ids = vec![];
    let mut empty_col_ids = vec![];
    let mut galaxies = vec![];

    // Check for empty rows/cols to duplicate
    for (i, row) in map.iter().enumerate() {
        if row.iter().all(|&c| c == b'.') {
            empty_row_ids.push(i)
        }
    }

    for j in 0..map[0].len() {
        let mut empty = true;

        for i in 0..map.len() {
            if map[i][j] == b'#' {
                empty = false;
                galaxies.push((i as i64, j as i64));
            }
        }

        if empty {
            empty_col_ids.push(j);
        }
    }

    // Get unique pairs of galaxies
    galaxies
        .iter()
        .combinations(2)
        .map(|galaxies| (galaxies[0], galaxies[1]))
        .map(|(a, b)| {
            let mut galaxy1_x = a.0;
            let mut galaxy2_x = b.0;

            for &row_id in &empty_row_ids {
                if a.0 > row_id as i64 {
                    galaxy1_x += expansion as i64 - 1;
                }

                if b.0 > row_id as i64 {
                    galaxy2_x += expansion as i64 - 1;
                }
            }

            let mut galaxy1_y = a.1;
            let mut galaxy2_y = b.1;

            for &col_id in &empty_col_ids {
                if a.1 > col_id as i64 {
                    galaxy1_y += expansion as i64 - 1;
                }

                if b.1 > col_id as i64 {
                    galaxy2_y += expansion as i64 - 1;
                }
            }

            let x = (galaxy1_x - galaxy2_x).abs();
            let y = (galaxy1_y - galaxy2_y).abs();

            x + y
        })
        .sum()
}

fn print(map: &[Vec<u8>]) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            eprint!("{}", map[i][j] as char);
        }

        eprintln!();
    }

    eprintln!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let input = indoc::indoc! {"
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
        "};

        assert_eq!(p1(input), 374);
    }

    #[test]
    fn p2_test() {
        let input = indoc::indoc! {"
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
        "};

        assert_eq!(p2(input, 1), 374);
        assert_eq!(p2(input, 10), 1030);
        assert_eq!(p2(input, 100), 8410);
    }
}
