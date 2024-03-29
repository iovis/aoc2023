use itertools::Itertools;

use self::dir::{Direction, DirectionExt};
use self::node::{Node, State};
use self::point::Point;

mod dir;
mod node;
mod point;

fn main() {
    let input = include_str!("input.txt");

    println!("p1 = {:?}", p1(input));
    println!("p2 = {:?}", p2(input));
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

#[allow(clippy::too_many_lines)]
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

    // print(&map);

    // Build expanded map
    let mut big_map: Vec<Vec<Node>> = vec![vec![Node::new(b'.'); map[0].len() * 3]; map.len() * 3];

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            // xxxxxxxxx
            // xoxxoxxox
            // xxxxxxxxx
            // 0 => 1
            // 1 => 4
            // 2 => 7 = i*3 + 1
            let (ii, jj) = (3 * i + 1, 3 * j + 1);
            big_map[ii][jj] = map[i][j];

            if map[i][j].state == State::Path {
                // I'm sorry, I can't be bothered right now
                match map[i][j].symbol {
                    b'|' => {
                        big_map[ii - 1][jj] = map[i][j];
                        big_map[ii + 1][jj] = map[i][j];
                    }
                    b'-' => {
                        big_map[ii][jj - 1] = map[i][j];
                        big_map[ii][jj + 1] = map[i][j];
                    }
                    b'L' => {
                        big_map[ii - 1][jj] = Node {
                            symbol: b'|',
                            visited: false,
                            state: State::Path,
                        };
                        big_map[ii][jj + 1] = Node {
                            symbol: b'-',
                            visited: false,
                            state: State::Path,
                        };
                    }
                    b'J' => {
                        big_map[ii - 1][jj] = Node {
                            symbol: b'|',
                            visited: false,
                            state: State::Path,
                        };
                        big_map[ii][jj - 1] = Node {
                            symbol: b'-',
                            visited: false,
                            state: State::Path,
                        };
                    }
                    b'7' => {
                        big_map[ii + 1][jj] = Node {
                            symbol: b'|',
                            visited: false,
                            state: State::Path,
                        };
                        big_map[ii][jj - 1] = Node {
                            symbol: b'-',
                            visited: false,
                            state: State::Path,
                        };
                    }
                    b'F' => {
                        big_map[ii + 1][jj] = Node {
                            symbol: b'|',
                            visited: false,
                            state: State::Path,
                        };
                        big_map[ii][jj + 1] = Node {
                            symbol: b'-',
                            visited: false,
                            state: State::Path,
                        };
                    }
                    b'S' => {
                        // CHEAT HARD
                        big_map[ii + 1][jj] = Node {
                            symbol: b'|',
                            visited: false,
                            state: State::Path,
                        };
                        big_map[ii - 1][jj] = Node {
                            symbol: b'|',
                            visited: false,
                            state: State::Path,
                        };
                        big_map[ii][jj - 1] = Node {
                            symbol: b'-',
                            visited: false,
                            state: State::Path,
                        };
                        big_map[ii][jj + 1] = Node {
                            symbol: b'-',
                            visited: false,
                            state: State::Path,
                        };
                    }
                    _ => (),
                }
            }
        }
    }

    // print(&big_map);

    bfs(&mut big_map, Point(0, 0));

    // print(&big_map);

    // Translate state of points back
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let (ii, jj) = (3 * i + 1, 3 * j + 1);
            map[i][j].state = big_map[ii][jj].state;
        }
    }

    // Rest of the points should be inside
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j].state == State::Unknown {
                map[i][j].state = State::In;
            }
        }
    }

    // print(&map);

    map.iter()
        .flatten()
        .filter(|node| node.state == State::In)
        .count()
}

fn bfs(map: &mut [Vec<Node>], start: Point) {
    let limits = (map.len() - 1, map[0].len() - 1);

    // Explore all directions
    for dir in Direction::iter() {
        let Some(next_point) = start.go(dir, limits) else {
            continue;
        };

        let (i, j) = next_point.coords();

        if map[i][j].visited || map[i][j].symbol == b'S' {
            continue;
        }

        if map[i][j].state == State::Unknown {
            map[i][j].state = State::Out;
            map[i][j].visited = true;

            bfs(map, next_point);
        }
    }
}

fn print(map: &[Vec<Node>]) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            eprint!("{}", map[i][j]);
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
            ..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ...
        "};

        assert_eq!(p1(input), 8);
    }

    #[test]
    fn p2_test1() {
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

        assert_eq!(p2(input), 4);
    }

    #[test]
    fn p2_test2() {
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

        assert_eq!(p2(input), 4);
    }

    #[test]
    fn p2_test3() {
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

        assert_eq!(p2(input), 8);
    }

    #[test]
    fn p2_test4() {
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

        assert_eq!(p2(input), 10);
    }
}
