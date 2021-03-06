use super::Card;
use druid::Data;
use std::fmt;
use std::rc::Rc;

#[derive(Clone, Data, Debug, PartialEq)]
pub struct Cascade(Rc<Vec<Card>>);

impl Cascade {
    pub fn new(cards: Vec<Card>) -> Self {
        Self(Rc::new(cards))
    }

    pub fn empty() -> Self {
        Self::new(Vec::new())
    }

    pub fn pop(&mut self) -> Option<Card> {
        Rc::get_mut(&mut self.0).unwrap().pop()
    }

    pub fn is_legal(&self, card: &Card) -> bool {
        match self.0.last() {
            None => true,
            Some(last_card) => {
                card.get_suit().is_red() != last_card.get_suit().is_red()
                    && card.get_rank() == last_card.get_rank() - 1
            }
        }
    }

    pub fn push(&mut self, card: Card) -> Result<(), (Card, &'static str)> {
        if self.is_legal(&card) {
            self.push_unchecked(card);
            Ok(())
        } else {
            Err((card, "That card cannot go on that cascade."))
        }
    }

    pub fn push_unchecked(&mut self, card: Card) {
        Rc::get_mut(&mut self.0).unwrap().push(card)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_sequential(&self) -> bool {
        self.0
            .iter()
            .try_fold(13, |prev, card| {
                if card.get_rank() > prev {
                    Err(())
                } else {
                    Ok(card.get_rank())
                }
            })
            .is_ok()
    }

    pub fn cards(&self) -> &Vec<Card> {
        &self.0
    }

    pub fn take(&mut self, count: usize) -> Vec<Card> {
        Rc::get_mut(&mut self.0)
            .map(|cards| cards.split_off(cards.len() - count))
            .unwrap()
    }
}

impl fmt::Display for Cascade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for card in self.cards() {
            writeln!(f, "{}", card)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::Suit;
    use super::{Card, Cascade};
    use std::rc::Rc;

    #[test]
    fn new() {
        assert_eq!(
            Cascade(Rc::new(vec![Card::new(1, Suit::Hearts)])),
            Cascade::new(vec![Card::new(1, Suit::Hearts)]),
        );
    }

    #[test]
    fn empty() {
        assert_eq!(Cascade(Rc::new(Vec::new())), Cascade::empty());
    }

    #[test]
    fn pop() {
        let mut cascade = Cascade::new(vec![
            Card::new(1, Suit::Hearts),
            Card::new(6, Suit::Diamonds),
        ]);

        assert_eq!(Some(Card::new(6, Suit::Diamonds)), cascade.pop());
        assert_eq!(Some(Card::new(1, Suit::Hearts)), cascade.pop());
        assert_eq!(None, cascade.pop());
    }

    #[test]
    fn push_empty() {
        let mut cascade = Cascade::empty();
        let card = Card::new(1, Suit::Hearts);

        assert!(cascade.is_legal(&card));
        assert_eq!(Ok(()), cascade.push(card));

        assert_eq!(Cascade::new(vec![Card::new(1, Suit::Hearts)]), cascade);
    }

    #[test]
    fn push_legal() {
        let mut cascade = Cascade::new(vec![Card::new(13, Suit::Clubs)]);
        let card = Card::new(12, Suit::Hearts);

        assert!(cascade.is_legal(&card));
        assert_eq!(Ok(()), cascade.push(card));

        assert_eq!(
            Cascade::new(vec![
                Card::new(13, Suit::Clubs),
                Card::new(12, Suit::Hearts),
            ]),
            cascade,
        );
    }

    #[test]
    fn push_illegal_color() {
        let mut cascade = Cascade::new(vec![Card::new(13, Suit::Clubs)]);
        let card = Card::new(12, Suit::Spades);

        assert_eq!(false, cascade.is_legal(&card));
        assert_eq!(
            Err((
                Card::new(12, Suit::Spades),
                "That card cannot go on that cascade.",
            )),
            cascade.push(card),
        );

        assert_eq!(Cascade::new(vec![Card::new(13, Suit::Clubs)]), cascade,);
    }

    #[test]
    fn push_illegal_rank() {
        let mut cascade = Cascade::new(vec![Card::new(13, Suit::Clubs)]);
        let card = Card::new(11, Suit::Hearts);

        assert_eq!(false, cascade.is_legal(&card));
        assert_eq!(
            Err((
                Card::new(11, Suit::Hearts),
                "That card cannot go on that cascade.",
            )),
            cascade.push(card),
        );

        assert_eq!(Cascade::new(vec![Card::new(13, Suit::Clubs)]), cascade,);
    }

    #[test]
    fn push_unchecked() {
        let mut cascade = Cascade::new(vec![Card::new(13, Suit::Clubs)]);
        let card = Card::new(11, Suit::Hearts);

        cascade.push_unchecked(card);

        assert_eq!(
            Cascade::new(vec![
                Card::new(13, Suit::Clubs),
                Card::new(11, Suit::Hearts),
            ]),
            cascade,
        );
    }

    #[test]
    fn len() {
        assert_eq!(0, Cascade::empty().len());
        assert_eq!(
            2,
            Cascade::new(vec![
                Card::new(13, Suit::Clubs),
                Card::new(12, Suit::Hearts)
            ])
            .len(),
        );
    }

    #[test]
    fn is_sequential() {
        let cascade = Cascade::new(vec![
            Card::new(13, Suit::Clubs),
            Card::new(10, Suit::Hearts),
            Card::new(9, Suit::Diamonds),
            Card::new(1, Suit::Spades),
        ]);
        assert!(cascade.is_sequential());

        let cascade = Cascade::new(vec![
            Card::new(10, Suit::Diamonds),
            Card::new(10, Suit::Hearts),
        ]);
        assert!(cascade.is_sequential());

        let cascade = Cascade::empty();
        assert!(cascade.is_sequential());

        let cascade = Cascade::new(vec![
            Card::new(1, Suit::Diamonds),
            Card::new(2, Suit::Hearts),
        ]);
        assert_eq!(false, cascade.is_sequential());
    }

    #[test]
    fn cards() {
        assert_eq!(
            &vec![Card::new(13, Suit::Clubs)],
            Cascade::new(vec![Card::new(13, Suit::Clubs)]).cards(),
        );
    }

    #[test]
    fn take() {
        let mut cascade = Cascade::new(vec![
            Card::new(4, Suit::Clubs),
            Card::new(3, Suit::Spades),
            Card::new(2, Suit::Hearts),
            Card::new(1, Suit::Diamonds),
        ]);

        assert_eq!(
            vec![Card::new(2, Suit::Hearts), Card::new(1, Suit::Diamonds)],
            cascade.take(2),
        );

        assert_eq!(
            Cascade::new(vec![Card::new(4, Suit::Clubs), Card::new(3, Suit::Spades)]),
            cascade,
        );
    }

    #[test]
    #[should_panic]
    fn take_invalid() {
        let mut cascade = Cascade::new(vec![Card::new(1, Suit::Hearts)]);
        cascade.take(2);
    }
}
