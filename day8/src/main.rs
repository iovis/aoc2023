use itertools::Itertools;

mod map;

fn main() {
    let input = include_str!("input.txt");

    println!("p1 = {:?}", p1(input));
    println!("p2 = {:?}", p2(input));
}

/// Follow the map from AAA to ZZZ
///
/// Instructions: L or R for left or right (if you run out, repeat)
/// Nodes: AAA = (BBB, CCC)
///
/// How many steps?
fn p1(input: &str) -> usize {
    let (instructions, map) = map::parse(input);
    let mut current = "AAA";

    for (step, dir) in instructions.iter().cycle().enumerate() {
        let dir = match dir {
            b'L' => 0,
            b'R' => 1,
            _ => unreachable!(),
        };

        current = map.get(current).unwrap()[dir];

        if current == "ZZZ" {
            return 1 + step;
        }
    }

    unreachable!()
}

/// Follow the map from keys ending in A till all keys end in Z
///
/// How many steps?
fn p2(input: &str) -> usize {
    let (instructions, map) = map::parse(input);
    let starting_nodes = map
        .keys()
        .filter(|node| node.ends_with('A'))
        .copied()
        .collect_vec();

    // let paths =
    starting_nodes
        .iter()
        // How long does it take for each node to reach 'Z'?
        .map(|&node| {
            let mut dir_iter = instructions.iter().cycle().enumerate();
            let mut current = node;

            while !current.ends_with('Z') {
                let dir = match dir_iter.next().unwrap().1 {
                    b'L' => 0,
                    b'R' => 1,
                    _ => unreachable!(),
                };

                current = map.get(current).unwrap()[dir];
            }

            dir_iter.next().unwrap().0
        })
        // Return the LCM of the paths
        .fold(1, lcm)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let input = indoc::indoc! {"
            RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
        "};

        assert_eq!(p1(input), 2);

        let input = indoc::indoc! {"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        "};

        assert_eq!(p1(input), 6);
    }

    #[test]
    fn p2_test() {
        let input = indoc::indoc! {"
            LR

            AAA = (AAB, XXX)
            AAB = (XXX, AAZ)
            AAZ = (AAB, XXX)
            BBA = (BBB, XXX)
            BBB = (BBC, BBC)
            BBC = (BBZ, BBZ)
            BBZ = (BBB, BBB)
            XXX = (XXX, XXX)
        "};

        assert_eq!(p2(input), 6);
    }
}
