use rand::seq::SliceRandom;
use rand::thread_rng;
use std::iter;

#[derive(Debug)]
pub struct Deck(Vec<Card>);

impl Deck {
    pub fn new(cards: Vec<Card>) -> Self {
        Self(cards)
    }

    /// For funsies, define a "fresh" deck according to the sequence used by Bicycle.
    pub fn fresh() -> Self {
        Self(
            iter::empty::<Card>()
                .chain((0..13).map(|i| Card::new(i + 1, Suit::Spades)))
                .chain((0..13).map(|i| Card::new(i + 1, Suit::Diamonds)))
                .chain((0..13).map(|i| Card::new(13 - i, Suit::Clubs)))
                .chain((0..13).map(|i| Card::new(13 - i, Suit::Hearts)))
                .collect(),
        )
    }

    pub fn shuffled() -> Self {
        let mut deck = Self::fresh();
        deck.shuffle();
        deck
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.0.shuffle(&mut rng);
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.0.pop()
    }
}

#[cfg(test)]
mod test_deck {
    use super::{Card, Deck, Suit};

    #[test]
    fn fresh_pop() {
        let mut deck = Deck::fresh();

        for i in 1..=13 {
            assert_eq!(Some(Card(i, Suit::Hearts)), deck.pop());
        }
        for i in 1..=13 {
            assert_eq!(Some(Card(i, Suit::Clubs)), deck.pop());
        }
        for i in (1..=13).rev() {
            assert_eq!(Some(Card(i, Suit::Diamonds)), deck.pop());
        }
        for i in (1..=13).rev() {
            assert_eq!(Some(Card(i, Suit::Spades)), deck.pop());
        }
        assert_eq!(None, deck.pop());
    }

    #[test]
    fn shuffled() {
        let deck = Deck::shuffled();
        assert_eq!(format!("{:?}", deck), format!("{:?}", deck));

        // Statistically, the chances of this failing are 1:(52!)
        assert_ne!(
            format!("{:?}", Deck::shuffled()),
            format!("{:?}", Deck::shuffled()),
        );
    }
}

#[derive(Debug, PartialEq)]
pub struct Card(u8, Suit);

impl Card {
    pub fn new(rank: u8, suit: Suit) -> Self {
        if rank >= 1 && rank <= 13 {
            Self(rank, suit)
        } else {
            panic!("Invalid rank.")
        }
    }

    pub fn get_rank(&self) -> u8 {
        self.0
    }

    pub fn get_suit(&self) -> Suit {
        self.1
    }
}

#[cfg(test)]
mod test_card {
    use super::{Card, Suit};

    #[test]
    fn new_valid() {
        assert_eq!(Card(1, Suit::Spades), Card::new(1, Suit::Spades));
        assert_eq!(Card(2, Suit::Hearts), Card::new(2, Suit::Hearts));
        assert_eq!(Card(12, Suit::Diamonds), Card::new(12, Suit::Diamonds));
        assert_eq!(Card(13, Suit::Clubs), Card::new(13, Suit::Clubs));
    }

    #[test]
    #[should_panic(expected = "Invalid rank.")]
    fn new_too_small() {
        Card::new(0, Suit::Spades);
    }

    #[test]
    #[should_panic(expected = "Invalid rank.")]
    fn new_too_big() {
        Card::new(14, Suit::Clubs);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    pub fn is_red(&self) -> bool {
        match self {
            Self::Diamonds | Self::Hearts => true,
            Self::Clubs | Self::Spades => false,
        }
    }
}

#[cfg(test)]
mod test_suit {
    use super::Suit;

    #[test]
    fn is_red() {
        assert_eq!(false, Suit::Clubs.is_red());
        assert_eq!(false, Suit::Spades.is_red());
        assert_eq!(true, Suit::Diamonds.is_red());
        assert_eq!(true, Suit::Hearts.is_red());
    }
}
