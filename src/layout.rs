use super::{Card, Deck, Suit};
use std::convert::TryInto;
use std::iter;

#[derive(Debug)]
pub struct Tableau {
    pub cells: [Cell; 4],
    pub foundations: [Foundation; 4],
    pub cascades: [Cascade; 8],
}

impl Tableau {
    pub fn empty() -> Self {
        Self {
            cells: (0..4)
                .map(|_| Cell::empty())
                .collect::<Vec<Cell>>()
                .try_into()
                .unwrap(),
            foundations: (0..4)
                .map(|_| Foundation::empty())
                .collect::<Vec<Foundation>>()
                .try_into()
                .unwrap(),
            cascades: (0..8)
                .map(|_| Cascade::empty())
                .collect::<Vec<Cascade>>()
                .try_into()
                .unwrap(),
        }
    }

    pub fn deal(mut deck: Deck) -> Self {
        let mut tableau = Self::empty();

        iter::from_fn(|| deck.pop())
            .zip((0..8).cycle())
            .for_each(|(card, i)| tableau.cascades[i].push_unchecked(card));

        tableau
    }

    pub fn is_won(&self) -> bool {
        self.cascades.iter().all(|cascade| cascade.is_sequential())
    }
}

#[cfg(test)]
mod test_tableau {
    use super::{Card, Cell, Deck, Foundation, Suit, Tableau};

    #[test]
    fn deal() {
        let tableau = Tableau::deal(Deck::shuffled());
        tableau
            .cells
            .iter()
            .for_each(|cell| assert_eq!(&Cell(None), cell));
        tableau.foundations.iter().for_each(|foundation| {
            assert_eq!(
                &Foundation {
                    suit: None,
                    cards: Vec::new()
                },
                foundation
            )
        });
        tableau.cascades[0..4]
            .iter()
            .for_each(|cascade| assert_eq!(7, cascade.len()));
        tableau.cascades[4..8]
            .iter()
            .for_each(|cascade| assert_eq!(6, cascade.len()));

        assert_eq!(
            52,
            tableau
                .cascades
                .iter()
                .fold(0, |i, cascade| i + cascade.len()),
        );
    }

    #[test]
    fn is_not_won_fresh() {
        let tableau = Tableau::deal(Deck::fresh());
        assert_eq!(false, tableau.is_won());
    }

    #[test]
    fn is_won_cascades() {
        let mut cards = Vec::<Card>::with_capacity(52);

        for i in 0..52 {
            cards.push(Card::new(
                (i / 4) + 1,
                match i % 4 {
                    0 => Suit::Spades,
                    1 => Suit::Diamonds,
                    2 => Suit::Hearts,
                    3 => Suit::Clubs,
                    _ => unreachable!(),
                },
            ));
        }

        let deck = Deck::new(cards);
        let tableau = Tableau::deal(deck);

        assert!(tableau.is_won(), format!("{:?}", tableau));

        // Just to be sure: we have aces on top, right?
        assert_eq!(
            Some(&Card::new(1, Suit::Clubs)),
            tableau.cascades[0].0.last(),
        );
    }

    #[test]
    fn is_won_empty() {
        assert!(Tableau::empty().is_won());
    }
}

#[derive(Debug, PartialEq)]
pub struct Cell(Option<Card>);

impl Cell {
    pub fn new(card: Card) -> Self {
        Self(Some(card))
    }

    pub fn empty() -> Self {
        Self(None)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub fn push(&mut self, card: Card) -> Result<(), (Card, &'static str)> {
        if self.is_empty() {
            self.0 = Some(card);
            Ok(())
        } else {
            Err((card, "A card is already present on that cell."))
        }
    }

    pub fn take(&mut self) -> Option<Card> {
        self.0.take()
    }
}

#[cfg(test)]
mod test_cell {
    use super::{Card, Cell, Suit};

    #[test]
    fn new() {
        assert_eq!(Cell(some_card()), Cell::new(card()));
    }

    #[test]
    fn empty() {
        assert_eq!(Cell(None), Cell::empty(),);
    }

    #[test]
    fn is_empty() {
        assert!(Cell(None).is_empty());
    }

    #[test]
    fn is_not_empty() {
        assert_eq!(false, Cell(some_card()).is_empty());
    }

    #[test]
    fn push_empty() {
        let mut cell = Cell(None);
        assert_eq!(Ok(()), cell.push(card()));
        assert_eq!(Cell(some_card()), cell);
    }

    #[test]
    fn push_not_empty() {
        let mut cell = Cell(some_card());
        assert_eq!(
            Err((card(), "A card is already present on that cell.")),
            cell.push(card()),
        );
    }

    #[test]
    fn take_empty() {
        let mut cell = Cell(None);
        assert_eq!(None, cell.take());
        assert_eq!(Cell(None), cell);
    }

    #[test]
    fn take_not_empty() {
        let mut cell = Cell(some_card());
        assert_eq!(some_card(), cell.take());
        assert_eq!(Cell(None), cell);
    }

    fn some_card() -> Option<Card> {
        Some(card())
    }

    fn card() -> Card {
        Card::new(1, Suit::Hearts)
    }
}

#[derive(Debug, PartialEq)]
pub struct Foundation {
    suit: Option<Suit>,
    cards: Vec<Card>,
}

impl Foundation {
    pub fn empty() -> Self {
        Self {
            suit: None,
            cards: Vec::new(),
        }
    }

    pub fn get_suit(&self) -> Option<Suit> {
        self.suit
    }

    pub fn get_rank(&self) -> u8 {
        self.cards.len().try_into().unwrap()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn is_legal(&self, card: &Card) -> bool {
        (self.suit == None || self.suit == Some(card.get_suit()))
            && self.get_rank() + 1 == card.get_rank()
    }

    pub fn push(&mut self, card: Card) -> Result<(), (Card, &'static str)> {
        if self.is_legal(&card) {
            self.suit = Some(card.get_suit());
            self.cards.push(card);
            Ok(())
        } else {
            Err((card, "That card is not valid on this foundation."))
        }
    }
}

#[cfg(test)]
mod test_foundation {
    use super::{Card, Foundation, Suit};

    #[test]
    fn empty() {
        assert_eq!(
            Foundation {
                suit: None,
                cards: Vec::new(),
            },
            Foundation::empty(),
        );
    }

    #[test]
    fn get_suit() {
        assert_eq!(
            Some(Suit::Spades),
            Foundation {
                suit: Some(Suit::Spades),
                cards: Vec::new()
            }
            .get_suit(),
        );

        assert_eq!(
            None,
            Foundation {
                suit: None,
                cards: Vec::new()
            }
            .get_suit(),
        );
    }

    #[test]
    fn push_empty_legal() {
        let mut foundation = Foundation::empty();
        let card = Card::new(1, Suit::Spades);

        assert!(foundation.is_legal(&card));
        assert_eq!(Ok(()), foundation.push(card));

        assert_eq!(
            Foundation {
                suit: Some(Suit::Spades),
                cards: vec![Card::new(1, Suit::Spades)]
            },
            foundation,
        );
    }

    #[test]
    fn push_empty_illegal_rank() {
        let mut foundation = Foundation::empty();
        let card = Card::new(2, Suit::Spades);

        assert_eq!(false, foundation.is_legal(&card));
        assert_eq!(
            Err((
                Card::new(2, Suit::Spades),
                "That card is not valid on this foundation.",
            )),
            foundation.push(card),
        );

        assert_eq!(Foundation::empty(), foundation);
    }

    #[test]
    fn push_nonempty_legal() {
        let mut foundation = Foundation {
            suit: Some(Suit::Clubs),
            cards: vec![Card::new(1, Suit::Clubs), Card::new(2, Suit::Clubs)],
        };
        let card = Card::new(3, Suit::Clubs);

        assert!(foundation.is_legal(&card));
        assert_eq!(Ok(()), foundation.push(card));

        assert_eq!(
            Foundation {
                suit: Some(Suit::Clubs),
                cards: vec![
                    Card::new(1, Suit::Clubs),
                    Card::new(2, Suit::Clubs),
                    Card::new(3, Suit::Clubs),
                ],
            },
            foundation,
        );
    }

    #[test]
    fn push_nonempty_illegal_rank() {
        let mut foundation = Foundation {
            suit: Some(Suit::Clubs),
            cards: vec![Card::new(1, Suit::Clubs)],
        };
        let card = Card::new(3, Suit::Clubs);

        assert_eq!(false, foundation.is_legal(&card));
        assert_eq!(
            Err((
                Card::new(3, Suit::Clubs),
                "That card is not valid on this foundation.",
            )),
            foundation.push(card),
        );

        assert_eq!(
            Foundation {
                suit: Some(Suit::Clubs),
                cards: vec![Card::new(1, Suit::Clubs)],
            },
            foundation,
        );
    }

    #[test]
    fn push_nonempty_illegal_suit() {
        let mut foundation = Foundation {
            suit: Some(Suit::Clubs),
            cards: vec![Card::new(1, Suit::Clubs)],
        };
        let card = Card::new(2, Suit::Hearts);

        assert_eq!(false, foundation.is_legal(&card));
        assert_eq!(
            Err((
                Card::new(2, Suit::Hearts),
                "That card is not valid on this foundation.",
            )),
            foundation.push(card),
        );

        assert_eq!(
            Foundation {
                suit: Some(Suit::Clubs),
                cards: vec![Card::new(1, Suit::Clubs)],
            },
            foundation,
        );
    }

    #[test]
    fn is_empty() {
        assert!(Foundation {
            suit: None,
            cards: Vec::new()
        }
        .is_empty());

        assert_eq!(
            false,
            Foundation {
                suit: Some(Suit::Clubs),
                cards: vec![Card::new(1, Suit::Clubs)]
            }
            .is_empty(),
        );
    }
}

#[derive(Debug, PartialEq)]
pub struct Cascade(Vec<Card>);

impl Cascade {
    pub fn new(cards: Vec<Card>) -> Self {
        Self(cards)
    }

    pub fn empty() -> Self {
        Self(Vec::new())
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.0.pop()
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
        self.0.push(card)
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
}

#[cfg(test)]
mod test_cascade {
    use super::{Card, Cascade, Suit};

    #[test]
    fn new() {
        assert_eq!(
            Cascade(vec![Card::new(1, Suit::Hearts)]),
            Cascade::new(vec![Card::new(1, Suit::Hearts)]),
        );
    }

    #[test]
    fn empty() {
        assert_eq!(Cascade(Vec::new()), Cascade::empty());
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
        let mut cascade = Cascade::new(Vec::new());
        let card = Card::new(1, Suit::Hearts);

        assert!(cascade.is_legal(&card));
        assert_eq!(Ok(()), cascade.push(card));

        assert_eq!(Cascade::new(vec![Card::new(1, Suit::Hearts)]), cascade,);
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
}
