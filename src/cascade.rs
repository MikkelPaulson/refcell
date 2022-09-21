use super::{Card, Rank};
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Cascade(Vec<Card>);

impl Cascade {
    pub fn new(cards: Vec<Card>) -> Self {
        Self(cards)
    }

    pub fn empty() -> Self {
        Self::new(Vec::new())
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.0.pop()
    }

    pub fn try_pop_stack(&mut self, count: usize) -> Option<Vec<Card>> {
        match count {
            0 => Some(Vec::new()),
            1 => self.pop().map(|card| vec![card]),
            c if c > self.len() => None,
            _ => {
                let mut card_iter = self.0[self.0.len() - count..].iter();
                let mut prev_card = card_iter.next().unwrap();

                while let Some(card) = card_iter.next() {
                    if !prev_card.is_legal(card) {
                        return None;
                    }

                    prev_card = card;
                }

                Some(self.pop_stack(count))
            }
        }
    }

    pub fn pop_stack(&mut self, count: usize) -> Vec<Card> {
        self.0.split_off(self.0.len() - count)
    }

    pub fn card_is_legal(&self, card: &Card) -> bool {
        self.0
            .last()
            .map_or(true, |last_card| last_card.is_legal(card))
    }

    pub fn stack_is_legal(&self, stack: &[Card]) -> bool {
        stack.first().map_or(true, |card| self.card_is_legal(card))
    }

    pub fn try_push(&mut self, card: Card) -> Result<(), (Card, &'static str)> {
        if self.card_is_legal(&card) {
            self.push(card);
            Ok(())
        } else {
            Err((card, "That card cannot go on that cascade."))
        }
    }

    pub fn push(&mut self, card: Card) {
        self.0.push(card)
    }

    pub fn try_push_stack(&mut self, stack: Vec<Card>) -> Result<(), (Vec<Card>, &'static str)> {
        if self.stack_is_legal(&stack) {
            self.push_stack(stack);
            Ok(())
        } else {
            Err((stack, "Those cards cannot go on that cascade."))
        }
    }

    pub fn push_stack(&mut self, mut stack: Vec<Card>) {
        self.0.append(&mut stack);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn is_sequential(&self) -> bool {
        self.0
            .iter()
            .try_fold(Rank::King, |prev, card| {
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
    use super::{Card, Cascade, Rank};

    #[test]
    fn new() {
        assert_eq!(
            Cascade(vec![Card::new(Rank::Ace, Suit::Hearts)]),
            Cascade::new(vec![Card::new(Rank::Ace, Suit::Hearts)]),
        );
    }

    #[test]
    fn empty() {
        assert_eq!(Cascade(Vec::new()), Cascade::empty());
    }

    #[test]
    fn pop() {
        let mut cascade = Cascade::new(vec![
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Six, Suit::Diamonds),
        ]);

        assert_eq!(Some(Card::new(Rank::Six, Suit::Diamonds)), cascade.pop());
        assert_eq!(Some(Card::new(Rank::Ace, Suit::Hearts)), cascade.pop());
        assert_eq!(None, cascade.pop());
    }

    #[test]
    fn try_push_empty() {
        let mut cascade = Cascade::empty();
        let card = Card::new(Rank::Ace, Suit::Hearts);

        assert!(cascade.card_is_legal(&card));
        assert_eq!(Ok(()), cascade.try_push(card));

        assert_eq!(
            Cascade::new(vec![Card::new(Rank::Ace, Suit::Hearts)]),
            cascade
        );
    }

    #[test]
    fn try_push_legal() {
        let mut cascade = Cascade::new(vec![Card::new(Rank::King, Suit::Clubs)]);
        let card = Card::new(Rank::Queen, Suit::Hearts);

        assert!(cascade.card_is_legal(&card));
        assert_eq!(Ok(()), cascade.try_push(card));

        assert_eq!(
            Cascade::new(vec![
                Card::new(Rank::King, Suit::Clubs),
                Card::new(Rank::Queen, Suit::Hearts),
            ]),
            cascade,
        );
    }

    #[test]
    fn try_push_illegal_color() {
        let mut cascade = Cascade::new(vec![Card::new(Rank::King, Suit::Clubs)]);
        let card = Card::new(Rank::Queen, Suit::Spades);

        assert_eq!(false, cascade.card_is_legal(&card));
        assert_eq!(
            Err((
                Card::new(Rank::Queen, Suit::Spades),
                "That card cannot go on that cascade.",
            )),
            cascade.try_push(card),
        );

        assert_eq!(
            Cascade::new(vec![Card::new(Rank::King, Suit::Clubs)]),
            cascade,
        );
    }

    #[test]
    fn try_push_illegal_rank() {
        let mut cascade = Cascade::new(vec![Card::new(Rank::King, Suit::Clubs)]);
        let card = Card::new(Rank::Jack, Suit::Hearts);

        assert_eq!(false, cascade.card_is_legal(&card));
        assert_eq!(
            Err((
                Card::new(Rank::Jack, Suit::Hearts),
                "That card cannot go on that cascade.",
            )),
            cascade.try_push(card),
        );

        assert_eq!(
            Cascade::new(vec![Card::new(Rank::King, Suit::Clubs)]),
            cascade,
        );
    }

    #[test]
    fn push() {
        let mut cascade = Cascade::new(vec![Card::new(Rank::King, Suit::Clubs)]);
        let card = Card::new(Rank::Jack, Suit::Hearts);

        cascade.push(card);

        assert_eq!(
            Cascade::new(vec![
                Card::new(Rank::King, Suit::Clubs),
                Card::new(Rank::Jack, Suit::Hearts),
            ]),
            cascade,
        );
    }

    #[test]
    fn len() {
        assert_eq!(0, Cascade::empty().len());
        assert!(Cascade::empty().is_empty());

        let nonempty = Cascade::new(vec![
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Hearts),
        ]);
        assert_eq!(2, nonempty.len());
        assert_eq!(false, nonempty.is_empty());
    }

    #[test]
    fn is_sequential() {
        let cascade = Cascade::new(vec![
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Hearts),
            Card::new(Rank::Nine, Suit::Diamonds),
            Card::new(Rank::Ace, Suit::Spades),
        ]);
        assert!(cascade.is_sequential());

        let cascade = Cascade::new(vec![
            Card::new(Rank::Ten, Suit::Diamonds),
            Card::new(Rank::Ten, Suit::Hearts),
        ]);
        assert!(cascade.is_sequential());

        let cascade = Cascade::empty();
        assert!(cascade.is_sequential());

        let cascade = Cascade::new(vec![
            Card::new(Rank::Ace, Suit::Diamonds),
            Card::new(Rank::Two, Suit::Hearts),
        ]);
        assert_eq!(false, cascade.is_sequential());
    }

    #[test]
    fn cards() {
        assert_eq!(
            &vec![Card::new(Rank::King, Suit::Clubs)],
            Cascade::new(vec![Card::new(Rank::King, Suit::Clubs)]).cards(),
        );
    }

    #[test]
    fn pop_stack() {
        let mut cascade = Cascade::new(vec![
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Two, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Diamonds),
        ]);

        assert_eq!(
            vec![
                Card::new(Rank::Two, Suit::Hearts),
                Card::new(Rank::Ace, Suit::Diamonds)
            ],
            cascade.pop_stack(2),
        );

        assert_eq!(
            Cascade::new(vec![
                Card::new(Rank::Four, Suit::Clubs),
                Card::new(Rank::Three, Suit::Spades)
            ]),
            cascade,
        );
    }

    #[test]
    #[should_panic]
    fn pop_stack_invalid() {
        let mut cascade = Cascade::new(vec![Card::new(Rank::Ace, Suit::Hearts)]);
        cascade.pop_stack(2);
    }
}
