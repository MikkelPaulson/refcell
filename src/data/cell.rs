use super::Card;
use druid::Data;

#[derive(Clone, Data, Debug, PartialEq)]
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

    pub fn peek(&self) -> Option<&Card> {
        self.0.as_ref()
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
mod tests {
    use super::super::Suit;
    use super::{Card, Cell};

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
        assert_eq!(false, Cell(some_card()).is_empty());
    }

    #[test]
    fn peek() {
        assert_eq!(None, Cell(None).peek());
        assert_eq!(Some(&card()), Cell(some_card()).peek());
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
