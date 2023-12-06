fn main() {
    let input = include_str!("input.txt");

    println!("p1 = {:?}", p1(input));
    println!("p2 = {:?}", p2(input));
}

/// Each race has a time and a current distance record
/// Each unit you hold to charge, you add one unit of velocity
/// => distance = v * t
///
/// Count the number of ways to break the record and multiply them
fn p1(input: &str) -> u64 {
    let races: Vec<_> = input
        .lines()
        .filter_map(|line| line.split(':').nth(1).map(str::trim))
        .map(|line| {
            line.split_whitespace()
                .filter_map(|n| n.parse::<u64>().ok())
                .collect::<Vec<_>>()
        })
        .collect();

    let races: Vec<(u64, u64)> = std::iter::zip(races[0].clone(), races[1].clone()).collect();

    races
        .iter()
        .map(|&(time, record)| {
            (1..=time)
                .map(|time_pressed| {
                    let velocity = time_pressed;
                    velocity * (time - time_pressed)
                })
                .filter(|&distance| distance > record)
                .count()
        })
        .product::<usize>()
        .try_into()
        .unwrap()
}

/// Now it's only one race
fn p2(input: &str) -> u64 {
    let race: Vec<_> = input
        .lines()
        .filter_map(|line| line.split(':').nth(1).map(str::trim))
        .map(|line| line.chars().filter(|c| c.is_numeric()).collect::<String>())
        .filter_map(|number| number.parse::<u64>().ok())
        .collect();

    let (time, record) = (race[0], race[1]);

    (1..=time)
        .map(|time_pressed| {
            let velocity = time_pressed;
            velocity * (time - time_pressed)
        })
        .filter(|&distance| distance > record)
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let input = indoc::indoc! {"
            Time:      7  15   30
            Distance:  9  40  200
        "};

        assert_eq!(p1(input), 288);
    }

    #[test]
    fn p2_test() {
        let input = indoc::indoc! {"
            Time:      7  15   30
            Distance:  9  40  200
        "};

        assert_eq!(p2(input), 71503);
    }
}
