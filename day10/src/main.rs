use itertools::Itertools;

use self::dir::{Direction, DirectionExt};
use self::point::Point;

mod dir;
mod point;

fn main() {
    let input = include_str!("input.txt");

    println!("p1 = {:?}", p1(input));
    // println!("p2 = {:?}", p2(input));
}

fn p1(input: &str) -> usize {
    let map = input.lines().map(str::as_bytes).collect_vec();
    let limits = (map.len() - 1, map[0].len() - 1);

    // Find Start of path
    let mut start = Point(0, 0);
    for (i, row) in map.iter().enumerate() {
        if let Some(j) = row.iter().position(|&x| x == b'S') {
            start = Point(i, j);
        }
    }

    let mut path = vec![start];
    let mut next_direction = Direction::North;

    // Calculate starting point direction
    for dir in Direction::iter() {
        let Some(next_point) = start.go(dir, limits) else {
            continue;
        };

        // -> East
        if let Some(next_dir) = next_point.symbol(&map).next_from(!dir) {
            path.push(next_point);
            next_direction = next_dir;
            break;
        }
    }

    loop {
        let current = *path.last().unwrap();
        let next_point = current.go(next_direction, limits).unwrap();

        if next_point.symbol(&map) == b'S' {
            break;
        }

        path.push(next_point);
        next_direction = next_point.symbol(&map).next_from(!next_direction).unwrap();
    }

    path.len() / 2
}

// #[derive(Debug, PartialEq, Eq)]
// enum S {
//     P,
//     O,
//     I,
//     X,
// }
//
// fn p2(input: &str) -> usize {
//     let mut map = input
//         .lines()
//         .map(|line| line.as_bytes().iter().map(|x| (x, S::X)).collect_vec())
//         .collect_vec();
//
//     let mut start = (0, 0);
//
//     for i in 0..map.len() {
//         if let Some(j) = map[i].iter().position(|(&x, _)| x == b'S') {
//             start = (i, j);
//             map[i][j].1 = S::P;
//         }
//     }
//
//     let mut path = vec![start];
//     let mut current = start;
//     let mut previous = (0, 0);
//
//     loop {
//         let (i, j) = current;
//         if path.len() == 1 {
//             let directions: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
//
//             for direction in directions {
//                 let (ii, jj) = (
//                     (i as i32 + direction.0).clamp(0, map.len() as i32) as usize,
//                     (j as i32 + direction.1).clamp(0, map[0].len() as i32) as usize,
//                 );
//
//                 let next = map[ii][jj].0;
//
//                 if direction == (1, 0) {
//                     match next {
//                         b'|' | b'7' | b'F' => {
//                             path.push((ii, jj));
//                             previous = (-1, 0);
//                             current = (ii, jj);
//                             map[i][j].1 = S::P;
//                             break;
//                         }
//                         _ => (),
//                     }
//                 } else if direction == (-1, 0) {
//                     match next {
//                         b'|' | b'L' | b'J' => {
//                             path.push((ii, jj));
//                             previous = (1, 0);
//                             current = (ii, jj);
//                             map[i][j].1 = S::P;
//                             break;
//                         }
//                         _ => (),
//                     }
//                 } else if direction == (0, 1) {
//                     match next {
//                         b'-' | b'7' | b'J' => {
//                             path.push((ii, jj));
//                             previous = (0, -1);
//                             current = (ii, jj);
//                             map[i][j].1 = S::P;
//                             break;
//                         }
//                         _ => (),
//                     }
//                 } else if direction == (0, -1) {
//                     match next {
//                         b'-' | b'L' | b'F' => {
//                             path.push((ii, jj));
//                             previous = (0, 1);
//                             current = (ii, jj);
//                             map[i][j].1 = S::P;
//                             break;
//                         }
//                         _ => (),
//                     }
//                 }
//             }
//         }
//
//         let (i, j) = current;
//         let mut direction = (0, 0);
//         for dir in directions_of(*map[i][j].0) {
//             if dir != previous {
//                 direction = dir;
//             }
//         }
//
//         previous = (-direction.0, -direction.1);
//         current = (
//             (i as i32 + direction.0).clamp(0, map.len() as i32) as usize,
//             (j as i32 + direction.1).clamp(0, map[0].len() as i32) as usize,
//         );
//
//         if *map[current.0][current.1].0 == b'S' {
//             break;
//         }
//
//         map[current.0][current.1].1 = S::P;
//         path.push(current);
//     }
//
//     for i in 0..map.len() {
//         for j in 0..map[i].len() {
//             let symbol = if map[i][j].1 == S::P { "*" } else { "·" };
//             print!("{symbol}");
//         }
//
//         println!();
//     }
//
//     23
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let input = indoc::indoc! {"
            ..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ...
        "};

        assert_eq!(p1(input), 8);
    }

    // #[test]
    // fn p2_test() {
    //     let input = indoc::indoc! {"
    //         ...........
    //         .S-------7.
    //         .|F-----7|.
    //         .||.....||.
    //         .||.....||.
    //         .|L-7.F-J|.
    //         .|..|.|..|.
    //         .L--J.L--J.
    //         ...........
    //     "};
    //
    //     println!("{input}");
    //     assert_eq!(p2(input), 4);
    //
    //     let input = indoc::indoc! {"
    //         ..........
    //         .S------7.
    //         .|F----7|.
    //         .||....||.
    //         .||....||.
    //         .|L-7F-J|.
    //         .|..||..|.
    //         .L--JL--J.
    //         ..........
    //     "};
    //
    //     assert_eq!(p2(input), 4);
    //
    //     let input = indoc::indoc! {"
    //         .F----7F7F7F7F-7....
    //         .|F--7||||||||FJ....
    //         .||.FJ||||||||L7....
    //         FJL7L7LJLJ||LJ.L-7..
    //         L--J.L7...LJS7F-7L7.
    //         ....F-J..F7FJ|L7L7L7
    //         ....L7.F7||L7|.L7L7|
    //         .....|FJLJ|FJ|F7|.LJ
    //         ....FJL-7.||.||||...
    //         ....L---J.LJ.LJLJ...
    //     "};
    //
    //     assert_eq!(p2(input), 8);
    //
    //     let input = indoc::indoc! {"
    //         FF7FSF7F7F7F7F7F---7
    //         L|LJ||||||||||||F--J
    //         FL-7LJLJ||||||LJL-77
    //         F--JF--7||LJLJ7F7FJ-
    //         L---JF-JLJ.||-FJLJJ7
    //         |F|F-JF---7F7-L7L|7|
    //         |FFJF7L7F-JF7|JL---7
    //         7-L-JL7||F7|L7F-7F7|
    //         L.L7LFJ|||||FJL7||LJ
    //         L7JLJL-JLJLJL--JLJ.L
    //     "};
    //
    //     assert_eq!(p2(input), 10);
    // }
}
