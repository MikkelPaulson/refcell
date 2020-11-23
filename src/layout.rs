use super::{Card, Deck, Suit};
use std::convert::TryInto;

#[derive(Debug)]
pub struct Tableau {
    cells: [Cell; 4],
    foundations: [Foundation; 4],
    cascades: [Cascade; 8],
}

impl Tableau {
    pub fn deal(mut deck: Deck) -> Self {
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
                .map(|col| {
                    Cascade::new(
                        (if col < 4 { 0..7 } else { 0..6 })
                            .map(|_| deck.pop().unwrap())
                            .collect::<Vec<Card>>(),
                    )
                })
                .collect::<Vec<Cascade>>()
                .try_into()
                .unwrap(),
        }
    }
}

#[cfg(test)]
mod test_tableau {
    use super::{Cell, Deck, Foundation, Tableau};

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
            .for_each(|cascade| assert_eq!(7, cascade.0.len()));
        tableau.cascades[4..8]
            .iter()
            .for_each(|cascade| assert_eq!(6, cascade.0.len()));
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

    pub fn take(&mut self) -> Option<Card> {
        self.0.take()
    }
}

#[derive(Debug, PartialEq)]
pub struct Foundation {
    suit: Option<Suit>,
    cards: Vec<Card>,
}

impl Foundation {
    pub fn new(suit: Suit, cards: Vec<Card>) -> Result<Foundation, &'static str> {
        for card in cards.iter() {
            if card.get_suit() != suit {
                return Err("Foundation cards must all be of the same suit");
            }
        }

        Ok(Self {
            suit: Some(suit),
            cards,
        })
    }

    pub fn empty() -> Self {
        Self {
            suit: None,
            cards: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Cascade(Vec<Card>);

impl Cascade {
    pub fn new(cards: Vec<Card>) -> Self {
        Self(cards)
    }
}
