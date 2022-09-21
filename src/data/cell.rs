use super::{Card, Single};
use std::fmt;

#[cfg(feature = "gui")]
use druid::{Data, Lens};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "gui", derive(Data, Lens))]
pub struct Cell {
    card: Option<Card>,
}

impl Cell {
    pub fn new(card: Card) -> Self {
        Self { card: Some(card) }
    }

    pub fn empty() -> Self {
        Self { card: None }
    }

    pub fn try_push(&mut self, card: Card) -> Result<(), (Card, &'static str)> {
        if self.is_empty() {
            self.card = Some(card);
            Ok(())
        } else {
            Err((card, "A card is already present on that cell."))
        }
    }

    pub fn take(&mut self) -> Option<Card> {
        self.card.take()
    }
}

impl Single for Cell {
    fn is_empty(&self) -> bool {
        self.card.is_none()
    }

    fn peek(&self) -> Option<&Card> {
        self.card.as_ref()
    }
}

impl fmt::Display for Cell {
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
    use super::super::{Rank, Suit};
    use super::{Card, Cell, Single};

    #[test]
    fn new() {
        assert_eq!(cell(some_card()), Cell::new(card()));
    }

    #[test]
    fn empty() {
        assert_eq!(cell(None), Cell::empty(),);
    }

    #[test]
    fn is_empty() {
        assert!(cell(None).is_empty());
        assert_eq!(false, cell(some_card()).is_empty());
    }

    #[test]
    fn peek() {
        assert_eq!(None, cell(None).peek());
        assert_eq!(Some(&card()), cell(some_card()).peek());
    }

    #[test]
    fn try_push_empty() {
        let mut test = cell(None);
        assert_eq!(Ok(()), test.try_push(card()));
        assert_eq!(cell(some_card()), test);
    }

    #[test]
    fn try_push_not_empty() {
        let mut test = cell(some_card());
        assert_eq!(
            Err((card(), "A card is already present on that cell.")),
            test.try_push(card()),
        );
    }

    #[test]
    fn take_empty() {
        let mut test = cell(None);
        assert_eq!(None, test.take());
        assert_eq!(cell(None), test);
    }

    #[test]
    fn take_not_empty() {
        let mut test = cell(some_card());
        assert_eq!(some_card(), test.take());
        assert_eq!(cell(None), test);
    }

    fn cell(card: Option<Card>) -> Cell {
        Cell { card }
    }

    fn some_card() -> Option<Card> {
        Some(card())
    }

    fn card() -> Card {
        Card::new(Rank::Ace, Suit::Hearts)
    }
}
