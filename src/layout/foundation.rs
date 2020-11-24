use super::{Card, Suit};

#[derive(Debug, PartialEq)]
pub struct Foundation(Vec<Card>);

impl Foundation {
    pub fn empty() -> Self {
        Self(Vec::new())
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
            self.0.push(card);
            Ok(())
        } else {
            Err((card, "That card is not valid on this foundation."))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Card, Foundation, Suit};

    #[test]
    fn empty() {
        assert_eq!(Foundation(Vec::new()), Foundation::empty());
    }

    #[test]
    fn get_suit() {
        assert_eq!(
            Some(Suit::Spades),
            Foundation(vec![Card::new(1, Suit::Spades)]).get_suit(),
        );

        assert_eq!(None, Foundation(Vec::new()).get_suit());
    }

    #[test]
    fn get_rank() {
        assert_eq!(0, Foundation(Vec::new()).get_rank());
        assert_eq!(
            2,
            Foundation(vec![Card::new(1, Suit::Clubs), Card::new(2, Suit::Clubs)]).get_rank(),
        );
    }

    #[test]
    fn peek_empty() {
        let foundation = Foundation(Vec::new());
        assert_eq!(None, foundation.peek());
    }

    #[test]
    fn peek_nonempty() {
        let foundation = Foundation(vec![Card::new(1, Suit::Clubs), Card::new(2, Suit::Clubs)]);
        assert_eq!(Some(&Card::new(2, Suit::Clubs)), foundation.peek());
        assert_eq!(Some(&Card::new(2, Suit::Clubs)), foundation.peek());
    }

    #[test]
    fn is_empty() {
        assert!(Foundation(Vec::new()).is_empty());

        assert_eq!(
            false,
            Foundation(vec![Card::new(1, Suit::Clubs)]).is_empty(),
        );
    }

    #[test]
    fn push_empty_legal() {
        let mut foundation = Foundation::empty();
        let card = Card::new(1, Suit::Spades);

        assert!(foundation.is_legal(&card));
        assert_eq!(Ok(()), foundation.push(card));

        assert_eq!(Foundation(vec![Card::new(1, Suit::Spades)]), foundation);
    }

    #[test]
    fn push_empty_illegal_rank() {
        let mut foundation = Foundation(Vec::new());
        let card = Card::new(2, Suit::Spades);

        assert_eq!(false, foundation.is_legal(&card));
        assert_eq!(
            Err((
                Card::new(2, Suit::Spades),
                "That card is not valid on this foundation.",
            )),
            foundation.push(card),
        );

        assert_eq!(Foundation(Vec::new()), foundation);
    }

    #[test]
    fn push_nonempty_legal() {
        let mut foundation = Foundation(vec![Card::new(1, Suit::Clubs), Card::new(2, Suit::Clubs)]);
        let card = Card::new(3, Suit::Clubs);

        assert!(foundation.is_legal(&card));
        assert_eq!(Ok(()), foundation.push(card));

        assert_eq!(
            Foundation(vec![
                Card::new(1, Suit::Clubs),
                Card::new(2, Suit::Clubs),
                Card::new(3, Suit::Clubs),
            ]),
            foundation,
        );
    }

    #[test]
    fn push_nonempty_illegal_rank() {
        let mut foundation = Foundation(vec![Card::new(1, Suit::Clubs)]);
        let card = Card::new(3, Suit::Clubs);

        assert_eq!(false, foundation.is_legal(&card));
        assert_eq!(
            Err((
                Card::new(3, Suit::Clubs),
                "That card is not valid on this foundation.",
            )),
            foundation.push(card),
        );

        assert_eq!(Foundation(vec![Card::new(1, Suit::Clubs)]), foundation);
    }

    #[test]
    fn push_nonempty_illegal_suit() {
        let mut foundation = Foundation(vec![Card::new(1, Suit::Clubs)]);
        let card = Card::new(2, Suit::Hearts);

        assert_eq!(false, foundation.is_legal(&card));
        assert_eq!(
            Err((
                Card::new(2, Suit::Hearts),
                "That card is not valid on this foundation.",
            )),
            foundation.push(card),
        );

        assert_eq!(Foundation(vec![Card::new(1, Suit::Clubs)]), foundation);
    }
}
