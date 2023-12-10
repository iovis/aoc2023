use itertools::Itertools;

use self::dir::{Direction, DirectionExt};
use self::node::{Node, State};
use self::point::Point;

mod dir;
mod node;
mod point;

fn main() {
    let input = include_str!("input.txt");

    // println!("p1 = {:?}", p1(input));
    println!("p2 = {:?}", p2(input));
}

// fn p1(input: &str) -> usize {
//     let map = input.lines().map(str::as_bytes).collect_vec();
//     let limits = (map.len() - 1, map[0].len() - 1);
//
//     // Find Start of path
//     let mut start = Point(0, 0);
//     for (i, row) in map.iter().enumerate() {
//         if let Some(j) = row.iter().position(|&x| x == b'S') {
//             start = Point(i, j);
//         }
//     }
//
//     let mut path = vec![start];
//     let mut next_direction = Direction::North;
//
//     // Calculate starting point direction
//     for dir in Direction::iter() {
//         let Some(next_point) = start.go(dir, limits) else {
//             continue;
//         };
//
//         // -> East
//         if let Some(next_dir) = next_point.symbol(&map).next_from(!dir) {
//             path.push(next_point);
//             next_direction = next_dir;
//             break;
//         }
//     }
//
//     loop {
//         let current = *path.last().unwrap();
//         let next_point = current.go(next_direction, limits).unwrap();
//
//         if next_point.symbol(&map) == b'S' {
//             break;
//         }
//
//         path.push(next_point);
//         next_direction = next_point.symbol(&map).next_from(!next_direction).unwrap();
//     }
//
//     path.len() / 2
// }

fn p2(input: &str) -> usize {
    let mut map = input
        .lines()
        .map(str::as_bytes)
        .map(|line| line.iter().map(|&c| Node::new(c)).collect_vec())
        .collect_vec();

    let limits = (map.len() - 1, map[0].len() - 1);

    // Find Start of path
    let mut start = Point(0, 0);
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j].symbol == b'S' {
                start = Point(i, j);
                map[i][j].state = State::Path;
            }
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
        let (i, j) = next_point.coords();
        if let Some(next_dir) = map[i][j].symbol.next_from(!dir) {
            path.push(next_point);
            map[i][j].state = State::Path;
            next_direction = next_dir;
            break;
        }
    }

    // Build the full path
    loop {
        let current = *path.last().unwrap();
        let next_point = current.go(next_direction, limits).unwrap();

        let (i, j) = next_point.coords();
        if map[i][j].symbol == b'S' {
            break;
        }

        path.push(next_point);
        map[i][j].state = State::Path;
        next_direction = map[i][j].symbol.next_from(!next_direction).unwrap();
    }

    print(&map);

    23
}

fn print(map: &[Vec<Node>]) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            print!("{}", map[i][j]);
        }

        println!();
    }
}

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

        // assert_eq!(p1(input), 8);
    }

    #[test]
    fn p2_test() {
        let input = indoc::indoc! {"
            ...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........
        "};

        println!("{input}");
        p2(input);
        println!("================");
        // assert_eq!(p2(input), 4);

        let input = indoc::indoc! {"
            ..........
            .S------7.
            .|F----7|.
            .||....||.
            .||....||.
            .|L-7F-J|.
            .|..||..|.
            .L--JL--J.
            ..........
        "};

        println!("{input}");
        p2(input);
        println!("================");
        // assert_eq!(p2(input), 4);

        let input = indoc::indoc! {"
            .F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...
        "};

        println!("{input}");
        p2(input);
        println!("================");
        // assert_eq!(p2(input), 8);

        let input = indoc::indoc! {"
            FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L
        "};

        println!("{input}");
        p2(input);
        // assert_eq!(p2(input), 10);
        assert!(false);
    }
}
