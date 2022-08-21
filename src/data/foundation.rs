use super::{Card, Rank, Single, Suit};
use std::fmt;
use std::rc::Rc;

#[cfg(feature = "gui")]
use druid::Data;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "gui", derive(Data))]
pub struct Foundation(Rc<Vec<Card>>);

impl Foundation {
    pub fn empty() -> Self {
        Self(Rc::new(Vec::new()))
    }

    pub fn get_suit(&self) -> Option<Suit> {
        self.peek().map(|card| card.get_suit())
    }

    pub fn get_rank(&self) -> Option<Rank> {
        self.peek().map(|card| card.get_rank())
    }

    pub fn is_legal(&self, card: &Card) -> bool {
        if card.get_rank() == Rank::Ace {
            self.is_empty()
        } else {
            self.peek().map_or(false, |foundation_card| {
                card.get_suit() == foundation_card.get_suit()
                    && card.get_rank() - 1 == foundation_card.get_rank()
            })
        }
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
            Err((card, "That card is not valid on that foundation."))
        }
    }
}

impl Single for Foundation {
    fn peek(&self) -> Option<&Card> {
        self.0.last()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
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
    use super::{Card, Foundation, Rank, Single, Suit};
    use std::rc::Rc;

    #[test]
    fn empty() {
        assert_eq!(make_foundation(Vec::new()), Foundation::empty());
    }

    #[test]
    fn get_suit() {
        assert_eq!(
            Some(Suit::Spades),
            make_foundation(vec![Card::new(Rank::Ace, Suit::Spades)]).get_suit(),
        );

        assert_eq!(None, make_foundation(Vec::new()).get_suit());
    }

    #[test]
    fn get_rank() {
        assert_eq!(None, make_foundation(Vec::new()).get_rank());
        assert_eq!(
            Some(Rank::Two),
            make_foundation(vec![
                Card::new(Rank::Ace, Suit::Clubs),
                Card::new(Rank::Two, Suit::Clubs)
            ])
            .get_rank(),
        );
    }

    #[test]
    fn peek_empty() {
        let foundation = make_foundation(Vec::new());
        assert_eq!(None, foundation.peek());
    }

    #[test]
    fn peek_nonempty() {
        let foundation = make_foundation(vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
        ]);
        assert_eq!(Some(&Card::new(Rank::Two, Suit::Clubs)), foundation.peek());
        assert_eq!(Some(&Card::new(Rank::Two, Suit::Clubs)), foundation.peek());
    }

    #[test]
    fn is_empty() {
        assert!(make_foundation(Vec::new()).is_empty());

        assert_eq!(
            false,
            make_foundation(vec![Card::new(Rank::Ace, Suit::Clubs)]).is_empty(),
        );
    }

    #[test]
    fn push_empty_legal() {
        let mut foundation = make_foundation(Vec::new());
        let card = Card::new(Rank::Ace, Suit::Spades);

        assert!(foundation.is_legal(&card));
        assert_eq!(Ok(()), foundation.push(card));

        assert_eq!(
            make_foundation(vec![Card::new(Rank::Ace, Suit::Spades)]),
            foundation
        );
    }

    #[test]
    fn push_empty_illegal_rank() {
        let mut foundation = make_foundation(Vec::new());
        let card = Card::new(Rank::Two, Suit::Spades);

        assert_eq!(false, foundation.is_legal(&card));
        assert_eq!(
            Err((
                Card::new(Rank::Two, Suit::Spades),
                "That card is not valid on that foundation.",
            )),
            foundation.push(card),
        );

        assert_eq!(make_foundation(Vec::new()), foundation);
    }

    #[test]
    fn push_nonempty_legal() {
        let mut foundation = make_foundation(vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
        ]);
        let card = Card::new(Rank::Three, Suit::Clubs);

        assert!(foundation.is_legal(&card));
        assert_eq!(Ok(()), foundation.push(card));

        assert_eq!(
            make_foundation(vec![
                Card::new(Rank::Ace, Suit::Clubs),
                Card::new(Rank::Two, Suit::Clubs),
                Card::new(Rank::Three, Suit::Clubs),
            ]),
            foundation,
        );
    }

    #[test]
    fn push_nonempty_illegal_rank() {
        let mut foundation = make_foundation(vec![Card::new(Rank::Ace, Suit::Clubs)]);
        let card = Card::new(Rank::Three, Suit::Clubs);

        assert_eq!(false, foundation.is_legal(&card));
        assert_eq!(
            Err((
                Card::new(Rank::Three, Suit::Clubs),
                "That card is not valid on that foundation.",
            )),
            foundation.push(card),
        );

        assert_eq!(
            make_foundation(vec![Card::new(Rank::Ace, Suit::Clubs)]),
            foundation
        );
    }

    #[test]
    fn push_nonempty_illegal_suit() {
        let mut foundation = make_foundation(vec![Card::new(Rank::Ace, Suit::Clubs)]);
        let card = Card::new(Rank::Two, Suit::Hearts);

        assert_eq!(false, foundation.is_legal(&card));
        assert_eq!(
            Err((
                Card::new(Rank::Two, Suit::Hearts),
                "That card is not valid on that foundation.",
            )),
            foundation.push(card),
        );

        assert_eq!(
            make_foundation(vec![Card::new(Rank::Ace, Suit::Clubs)]),
            foundation
        );
    }

    fn make_foundation(cards: Vec<Card>) -> Foundation {
        Foundation(Rc::new(cards))
    }
}
