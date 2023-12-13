use nom::bytes::complete::tag;
use nom::character::complete;
use nom::multi::separated_list1;
use nom::IResult;

fn parse_numbers(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(tag(","), complete::u64)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_numbers_test() {
        let input = "1,1,3";

        assert_eq!(parse_numbers(input), Ok(("", vec![1, 1, 3])));
    }

    // #[test]
    // fn parse_line_test() {
    //     let input = indoc::indoc! {"
    //         ???.### 1,1,3
    //         .??..??...?##. 1,1,3
    //         ?#?#?#?#?#?#?#? 1,3,1,6
    //         ????.#...#... 4,1,1
    //         ????.######..#####. 1,6,5
    //         ?###???????? 3,2,1
    //     "};
    //
    //     assert_eq!(p1(input), 21);
    // }
}
