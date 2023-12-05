use crate::RangeMapping;

#[derive(Debug, PartialEq, Eq)]
pub struct Map {
    pub source: String,
    pub destination: String,
    mappings: Vec<RangeMapping>,
}

impl Map {
    pub fn new(source: &str, destination: &str, mappings: &[RangeMapping]) -> Self {
        Self {
            source: source.into(),
            destination: destination.into(),
            mappings: mappings.to_owned(),
        }
    }

    pub fn find(&self, location: u64) -> u64 {
        self.mappings
            .iter()
            .find_map(|range| range.find(location))
            .unwrap_or(location)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_test() {
        let map = Map::new(
            "seed",
            "soil",
            &[
                RangeMapping {
                    source: 98,
                    destination: 50,
                    len: 2,
                },
                RangeMapping {
                    source: 50,
                    destination: 52,
                    len: 48,
                },
            ],
        );

        assert_eq!(map.find(1), 1);
        assert_eq!(map.find(49), 49);

        assert_eq!(map.find(50), 52);
        assert_eq!(map.find(51), 53);
        assert_eq!(map.find(97), 99);

        assert_eq!(map.find(98), 50);
        assert_eq!(map.find(99), 51);
    }
}
