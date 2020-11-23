#[derive(Debug, PartialEq)]
struct Card(u8, Suit);

impl Card {
    pub fn new(rank: u8, suit: Suit) -> Result<Card, &'static str> {
        if rank >= 1 && rank <= 13 {
            Ok(Card(rank, suit))
        } else {
            Err("Invalid rank")
        }
    }
}

#[cfg(test)]
mod test_card {
    use super::{Card, Suit};

    #[test]
    fn new() {
        assert_eq!(Err("Invalid rank"), Card::new(0, Suit::Hearts));
        assert_eq!(Ok(Card(1, Suit::Spades)), Card::new(1, Suit::Spades));
        assert_eq!(Ok(Card(2, Suit::Hearts)), Card::new(2, Suit::Hearts));
        assert_eq!(Ok(Card(12, Suit::Diamonds)), Card::new(12, Suit::Diamonds));
        assert_eq!(Ok(Card(13, Suit::Clubs)), Card::new(13, Suit::Clubs));
        assert_eq!(Err("Invalid rank"), Card::new(14, Suit::Clubs));
    }
}

#[derive(Debug, PartialEq)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    pub fn is_red(&self) -> bool {
        match self {
            Suit::Diamonds | Suit::Hearts => true,
            Suit::Clubs | Suit::Spades => false,
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
