use super::{Card, Suit};
use druid::Data;
use std::fmt;
use std::rc::Rc;

#[derive(Clone, Data, Debug, PartialEq)]
pub struct Foundation(Rc<Vec<Card>>);

impl Foundation {
    pub fn empty() -> Self {
        Self(Rc::new(Vec::new()))
    }

    pub fn get_suit(&self) -> Option<Suit> {
        self.peek().map(|card| card.get_suit())
    }

    pub fn get_rank(&self) -> u8 {
        self.peek().map(|card| card.get_rank()).unwrap_or(0)
    }

    pub fn peek(&self) -> Option<&Card> {
        self.0.last()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn is_legal(&self, card: &Card) -> bool {
        (self.get_suit() == None || self.get_suit() == Some(card.get_suit()))
            && self.get_rank() + 1 == card.get_rank()
    }

    pub fn push(&mut self, card: Card) -> Result<(), (Card, &'static str)> {
        if self.is_legal(&card) {
            if let Some(cards) = Rc::get_mut(&mut self.0) {
                cards.push(card);
                Ok(())
            } else {
                panic!("Could not modify foundation!");
            }
        } else {
            Err((card, "That card is not valid on this foundation."))
        }
    }
}

impl fmt::Display for Foundation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(card) = self.peek() {
            write!(f, "{}", card)
        } else {
            write!(f, "\x1b[2m🂠\x1b[0m ")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Card, Foundation, Suit};
    use std::rc::Rc;

    #[test]
    fn empty() {
        assert_eq!(make_foundation(Vec::new()), Foundation::empty());
    }

    #[test]
    fn get_suit() {
        assert_eq!(
            Some(Suit::Spades),
            make_foundation(vec![Card::new(1, Suit::Spades)]).get_suit(),
        );

        assert_eq!(None, make_foundation(Vec::new()).get_suit());
    }

    #[test]
    fn get_rank() {
        assert_eq!(0, make_foundation(Vec::new()).get_rank());
        assert_eq!(
            2,
            make_foundation(vec![Card::new(1, Suit::Clubs), Card::new(2, Suit::Clubs)]).get_rank(),
        );
    }

    #[test]
    fn peek_empty() {
        let foundation = make_foundation(Vec::new());
        assert_eq!(None, foundation.peek());
    }

    #[test]
    fn peek_nonempty() {
        let foundation =
            make_foundation(vec![Card::new(1, Suit::Clubs), Card::new(2, Suit::Clubs)]);
        assert_eq!(Some(&Card::new(2, Suit::Clubs)), foundation.peek());
        assert_eq!(Some(&Card::new(2, Suit::Clubs)), foundation.peek());
    }

    #[test]
    fn is_empty() {
        assert!(make_foundation(Vec::new()).is_empty());

        assert_eq!(
            false,
            make_foundation(vec![Card::new(1, Suit::Clubs)]).is_empty(),
        );
    }

    #[test]
    fn push_empty_legal() {
        let mut foundation = make_foundation(Vec::new());
        let card = Card::new(1, Suit::Spades);

        assert!(foundation.is_legal(&card));
        assert_eq!(Ok(()), foundation.push(card));

        assert_eq!(
            make_foundation(vec![Card::new(1, Suit::Spades)]),
            foundation
        );
    }

    #[test]
    fn push_empty_illegal_rank() {
        let mut foundation = make_foundation(Vec::new());
        let card = Card::new(2, Suit::Spades);

        assert_eq!(false, foundation.is_legal(&card));
        assert_eq!(
            Err((
                Card::new(2, Suit::Spades),
                "That card is not valid on this foundation.",
            )),
            foundation.push(card),
        );

        assert_eq!(make_foundation(Vec::new()), foundation);
    }

    #[test]
    fn push_nonempty_legal() {
        let mut foundation =
            make_foundation(vec![Card::new(1, Suit::Clubs), Card::new(2, Suit::Clubs)]);
        let card = Card::new(3, Suit::Clubs);

        assert!(foundation.is_legal(&card));
        assert_eq!(Ok(()), foundation.push(card));

        assert_eq!(
            make_foundation(vec![
                Card::new(1, Suit::Clubs),
                Card::new(2, Suit::Clubs),
                Card::new(3, Suit::Clubs),
            ]),
            foundation,
        );
    }

    #[test]
    fn push_nonempty_illegal_rank() {
        let mut foundation = make_foundation(vec![Card::new(1, Suit::Clubs)]);
        let card = Card::new(3, Suit::Clubs);

        assert_eq!(false, foundation.is_legal(&card));
        assert_eq!(
            Err((
                Card::new(3, Suit::Clubs),
                "That card is not valid on this foundation.",
            )),
            foundation.push(card),
        );

        assert_eq!(make_foundation(vec![Card::new(1, Suit::Clubs)]), foundation);
    }

    #[test]
    fn push_nonempty_illegal_suit() {
        let mut foundation = make_foundation(vec![Card::new(1, Suit::Clubs)]);
        let card = Card::new(2, Suit::Hearts);

        assert_eq!(false, foundation.is_legal(&card));
        assert_eq!(
            Err((
                Card::new(2, Suit::Hearts),
                "That card is not valid on this foundation.",
            )),
            foundation.push(card),
        );

        assert_eq!(make_foundation(vec![Card::new(1, Suit::Clubs)]), foundation);
    }

    fn make_foundation(cards: Vec<Card>) -> Foundation {
        Foundation(Rc::new(cards))
    }
}
