use day5::maps;
use rayon::prelude::*;

fn main() {
    let input = include_str!("input.txt");

    println!("p1 = {:?}", p1(input));
    println!("p2 = {:?}", p2(input));
}

/// Find the lowest location for the seeds
///
/// Maps are defined as:
/// <source>-to-<destination> map:
/// <destination_start> <source_start> <range>
///
/// Example:
/// ```
/// 50 98 2 => source_to_dest = [(98, 50), (99, 51)]
/// ```
///
/// If not mapped, source and dest are the same
fn p1(input: &str) -> u64 {
    let (seeds, maps) = maps::parse(input);

    seeds
        .iter()
        .map(|seed| {
            let mut current_source = "seed";
            let mut current_location = *seed;

            // Currently the Maps seem to be unique and in order,
            // so I can just iterate over it
            for map in &maps {
                assert!(current_source == map.source);
                current_location = map.find(current_location);
                current_source = &map.destination;
            }

            current_location
        })
        .min()
        .unwrap()
}

/// Same, but now `seeds:` is a range
///
/// Example:
/// ```
/// seeds: 79 14 55 13
/// => (79..79+14)
/// => (55..55+13)
/// ```
fn p2(input: &str) -> u64 {
    let (seeds, maps) = maps::parse(input);

    seeds
        // Split seeds in chunks to make generators for the seeds
        .chunks_exact(2)
        .par_bridge() // Turn into parallel iterator
        .flat_map(|chunk| {
            let start = chunk[0];
            let end = start + chunk[1];

            start..end
        })
        // Same as p1
        .map(|seed| {
            let mut current_source = "seed";
            let mut current_location = seed;

            // Currently the Maps seem to be unique and in order,
            // so I can just iterate over it
            for map in &maps {
                assert!(current_source == map.source);
                current_location = map.find(current_location);
                current_source = &map.destination;
            }

            current_location
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let input = indoc::indoc! {"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        "};

        assert_eq!(p1(input), 35);
    }

    #[test]
    fn p2_test() {
        let input = indoc::indoc! {"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        "};

        assert_eq!(p2(input), 46);
    }
}
