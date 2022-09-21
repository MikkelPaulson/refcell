use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;
use std::iter;

#[derive(Debug)]
pub struct Deck(Vec<Card>);

impl Deck {
    pub fn new(cards: Vec<Card>) -> Self {
        Self(cards)
    }

    /// For funsies, define a "fresh" deck according to the sequence used by Bicycle.
    pub fn fresh() -> Self {
        Self::new(
            iter::empty::<Card>()
                .chain((1..=13).map(|i| Card::new(i.try_into().unwrap(), Suit::Spades)))
                .chain((1..=13).map(|i| Card::new(i.try_into().unwrap(), Suit::Diamonds)))
                .chain(
                    (1..=13)
                        .rev()
                        .map(|i| Card::new(i.try_into().unwrap(), Suit::Clubs)),
                )
                .chain(
                    (1..=13)
                        .rev()
                        .map(|i| Card::new(i.try_into().unwrap(), Suit::Hearts)),
                )
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
            assert_eq!(Some(Card(i.try_into().unwrap(), Suit::Hearts)), deck.pop());
        }
        for i in 1..=13 {
            assert_eq!(Some(Card(i.try_into().unwrap(), Suit::Clubs)), deck.pop());
        }
        for i in (1..=13).rev() {
            assert_eq!(
                Some(Card(i.try_into().unwrap(), Suit::Diamonds)),
                deck.pop(),
            );
        }
        for i in (1..=13).rev() {
            assert_eq!(Some(Card(i.try_into().unwrap(), Suit::Spades)), deck.pop());
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Card(Rank, Suit);

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self(rank, suit)
    }

    pub fn get_rank(&self) -> Rank {
        self.0
    }

    pub fn get_suit(&self) -> Suit {
        self.1
    }

    pub fn is_legal(&self, other: &Card) -> bool {
        self.get_suit().is_red() != other.get_suit().is_red()
            && self.get_rank().try_decrement() == Some(other.get_rank())
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
\x1b[{color};7m{rank}{space}{suit}\x1b[0m
\x1b[{color};7m{suit}{space}{rank}\x1b[0m",
            color = if self.get_suit().is_red() { "91" } else { "39" },
            rank = self.get_rank(),
            space = if self.get_rank() == Rank::Ten {
                ""
            } else {
                " "
            },
            suit = self.get_suit(),
        )
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Rank {
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}

impl Rank {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ace => "A",
            Self::Two => "2",
            Self::Three => "3",
            Self::Four => "4",
            Self::Five => "5",
            Self::Six => "6",
            Self::Seven => "7",
            Self::Eight => "8",
            Self::Nine => "9",
            Self::Ten => "10",
            Self::Jack => "J",
            Self::Queen => "Q",
            Self::King => "K",
        }
    }

    pub fn try_increment(&self) -> Option<Rank> {
        (*self as u8 + 1).try_into().ok()
    }

    pub fn try_decrement(&self) -> Option<Rank> {
        (*self as u8 - 1).try_into().ok()
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<u8> for Rank {
    type Error = ();

    fn try_from(input: u8) -> Result<Self, Self::Error> {
        Ok(match input {
            1 => Self::Ace,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            9 => Self::Nine,
            10 => Self::Ten,
            11 => Self::Jack,
            12 => Self::Queen,
            13 => Self::King,
            _ => return Err(()),
        })
    }
}

impl From<Rank> for u8 {
    fn from(input: Rank) -> Self {
        input as Self
    }
}

/*
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\x1b[{}m{}\x1b[0m ",
            if self.get_suit().is_red() { "91" } else { "39" },
            char::from_u32(
                0x1f0a0
                    + if self.get_rank() < 12 {
                        self.get_rank()
                    } else {
                        self.get_rank() + 1
                    } as u32
                    + match self.get_suit() {
                        Suit::Spades => 0x00,
                        Suit::Hearts => 0x10,
                        Suit::Diamonds => 0x20,
                        Suit::Clubs => 0x30,
                    }
            )
            .unwrap()
        )
    }
}
*/

#[cfg(test)]
mod test_card {
    use super::{Card, Rank, Suit};

    #[test]
    fn new_valid() {
        assert_eq!(
            Card(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Spades),
        );
    }

    #[test]
    fn eq() {
        assert_eq!(Card(Rank::Ace, Suit::Hearts), Card(Rank::Ace, Suit::Hearts));
        assert_ne!(Card(Rank::Two, Suit::Hearts), Card(Rank::Ace, Suit::Hearts));
        assert_ne!(Card(Rank::Ace, Suit::Spades), Card(Rank::Ace, Suit::Hearts));
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

    pub fn as_char(&self) -> char {
        match self {
            Self::Spades => '\u{2660}',
            Self::Clubs => '\u{2663}',
            Self::Hearts => '\u{2665}',
            Self::Diamonds => '\u{2666}',
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.as_char())
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
