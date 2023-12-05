use crate::RangeMapping;

#[derive(Debug, PartialEq, Eq)]
pub struct Map {
    pub source: String,
    pub destination: String,
    pub mappings: Vec<RangeMapping>,
}

impl Map {
    pub fn find(&self, location: u64) -> u64 {
        for mapping in &self.mappings {
            if let Some(location) = mapping.find(location) {
                return location;
            }
        }

        // If not found, it's the same
        location
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_test() {
        let map = Map {
            source: "seed".to_string(),
            destination: "soil".to_string(),
            mappings: vec![
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
        };

        assert_eq!(map.find(1), 1);
        assert_eq!(map.find(49), 49);

        assert_eq!(map.find(50), 52);
        assert_eq!(map.find(51), 53);
        assert_eq!(map.find(97), 99);

        assert_eq!(map.find(98), 50);
        assert_eq!(map.find(99), 51);
    }
}
