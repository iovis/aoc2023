pub type ID = u32;

#[derive(Debug, PartialEq, Eq)]
pub struct Card {
    pub id: ID,
    pub winning_numbers: Vec<u32>,
    pub played_numbers: Vec<u32>,
}

impl Card {
    /// How many played numbers won? (Doesn't check for duplicates)
    /// For big numbers it would probably faster to use hashsets or order + binary search,
    /// but cache locality with something this small is probably faster and simpler
    pub fn score(&self) -> u32 {
        self.played_numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .count()
            .try_into()
            .unwrap()
    }
}
