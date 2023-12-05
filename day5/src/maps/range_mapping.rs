#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RangeMapping {
    pub source: u64,
    pub destination: u64,
    pub len: u64,
}

impl From<(u64, u64, u64)> for RangeMapping {
    fn from((destination, source, len): (u64, u64, u64)) -> Self {
        Self {
            source,
            destination,
            len,
        }
    }
}

impl RangeMapping {
    pub fn find(&self, location: u64) -> Option<u64> {
        if !(self.source..self.source + self.len).contains(&location) {
            return None;
        }

        let delta = location - self.source;

        Some(self.destination + delta)
    }

    // Seems to be too slow to try to memoize into a HashMap/BTreeMap,
    // even with fast hashing like rustc-hash
    pub fn iter(&self) -> impl Iterator<Item = (u64, u64)> {
        (self.source..)
            .zip(self.destination..)
            .take(self.len as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_test() {
        let range = RangeMapping {
            source: 98,
            destination: 50,
            len: 2,
        };

        assert_eq!(range.find(3), None);
        assert_eq!(range.find(98), Some(50));
        assert_eq!(range.find(99), Some(51));
    }

    #[test]
    fn range_mapping_test() {
        let range = RangeMapping::from((50, 98, 2));
        let mappings = range.iter().collect::<Vec<_>>();

        assert_eq!(mappings, vec![(98, 50), (99, 51)]);
    }
}
