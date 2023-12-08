mod map;

fn main() {
    let input = include_str!("input.txt");

    println!("p1 = {:?}", p1(input));
    // println!("p2 = {:?}", p2(input));
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
}
