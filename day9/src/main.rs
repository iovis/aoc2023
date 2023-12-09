use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");

    println!("p1 = {:?}", p1(input));
    println!("p2 = {:?}", p2(input));
}

fn p1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| line.split_whitespace())
        .map(|numbers| numbers.filter_map(|n| n.parse::<i64>().ok()).collect_vec())
        .fold(0, |total, line| total + predict(&line, false))
}

fn p2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| line.split_whitespace())
        .map(|numbers| numbers.filter_map(|n| n.parse::<i64>().ok()).collect_vec())
        .fold(0, |total, line| total + predict(&line, true))
}

fn predict(line: &[i64], backwards: bool) -> i64 {
    let mut diff = Vec::with_capacity(line.len() - 1);

    line.iter()
        .tuple_windows()
        .for_each(|(a, b)| diff.push(b - a));

    let mut previous = 0;
    if !diff.iter().all(|&x| x == 0) {
        previous = predict(&diff, backwards);
    }

    if backwards {
        line[0] - previous
    } else {
        line[line.len() - 1] + previous
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let input = indoc::indoc! {"
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        "};

        assert_eq!(p1(input), 114);
    }

    #[test]
    fn p2_test() {
        let input = indoc::indoc! {"
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        "};

        assert_eq!(p2(input), 2);
    }
}
