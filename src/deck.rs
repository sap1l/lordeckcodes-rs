use crate::card::{Card, CardCodeAndCount};
use crate::error::LorError;

/// Holds a set of [`CardCodeAndCount`].
///
/// [`CardCodeAndCount`]: struct.CardCodeAndCount.html
///
#[derive(Debug)]
pub struct Deck(Vec<CardCodeAndCount>);

impl Deck {
    /// Create a new empty `Deck`.
    pub fn new() -> Deck {
        Deck { 0: vec![] }
    }

    /// Create a new `Deck` from a `Vec` of `CardCodeAndCount`.
    ///
    /// # Examples
    /// ```
    /// use lordeckcodes::{Deck, CardCodeAndCount};
    /// let deck = Deck::from_vec(vec![
    ///     CardCodeAndCount::from_data("01SI015", 3).unwrap(),
    ///     CardCodeAndCount::from_data("01SI044", 3).unwrap(),
    ///     CardCodeAndCount::from_data("01SI048", 3).unwrap(),
    ///     CardCodeAndCount::from_data("01SI054", 3).unwrap(),
    /// ]);
    /// ```
    pub fn from_vec(vec: Vec<CardCodeAndCount>) -> Deck {
        Deck { 0: vec }
    }

    /// Add a `CardCodeAndCount` to the `Deck`.
    pub fn add(&mut self, card: CardCodeAndCount) {
        self.0.push(card);
    }

    /// Create and add a new `CardCodeAndCount` to the deck from the provided data.
    pub fn add_from_data(&mut self, code: &str, count: i32) -> Result<(), LorError> {
        let card = CardCodeAndCount::new(Card::from_code(code)?, count);
        Ok(self.add(card))
    }

    /// Obtain a reference to the list of `CardCodeAndCount`.
    pub fn cards(&self) -> &Vec<CardCodeAndCount> {
        &self.0
    }
}

impl PartialEq for Deck {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}