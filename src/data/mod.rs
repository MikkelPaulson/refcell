use druid::{Data, Lens};
use std::convert::TryInto;
use std::fmt;
use std::iter;

pub use action::{Action, Coordinate};
pub use cascade::Cascade;
pub use cell::Cell;
pub use deck::{Card, Deck, Suit};
pub use foundation::Foundation;
pub use single::Single;

mod action;
mod cascade;
mod cell;
mod deck;
mod foundation;
mod single;

#[derive(Clone, Data, Debug, Lens)]
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

    pub fn action(&mut self, action: Action) -> Result<(), &'static str> {
        if let Some(card) = match action.from {
            Coordinate::Cascade(n) => self.cascades[n as usize].pop(),
            Coordinate::Cell(n) => self.cells[n as usize].take(),
            Coordinate::Foundation(_) => return Err("You cannot take a card from a foundation."),
        } {
            if let Err((card, message)) = match action.to {
                Coordinate::Cascade(n) => self.cascades[n as usize].push(card),
                Coordinate::Cell(n) => self.cells[n as usize].push(card),
                Coordinate::Foundation(n) => self.foundations[n as usize].push(card),
            } {
                match action.from {
                    Coordinate::Cascade(n) => self.cascades[n as usize].push_unchecked(card),
                    Coordinate::Cell(n) => self.cells[n as usize].push(card).unwrap(),
                    Coordinate::Foundation(_) => unreachable!(),
                }

                Err(message)
            } else {
                Ok(())
            }
        } else {
            Err("That space is empty.")
        }
    }

    pub fn is_won(&self) -> bool {
        self.cascades.iter().all(|cascade| cascade.is_sequential())
    }
}

impl fmt::Display for Tableau {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ａｂｃｄ  ｗｘｙｚ")?;
        self.cells
            .iter()
            .try_for_each(|cell| write!(f, "{}", cell))?;
        write!(f, "  ")?;
        self.foundations
            .iter()
            .try_for_each(|cell| write!(f, "{}", cell))?;
        write!(f, "\n\n")?;
        writeln!(f, " １２３４５６７８")?;

        let longest_cascade = self
            .cascades
            .iter()
            .map(|cascade| cascade.len())
            .max()
            .unwrap_or(0);

        for row in 0..longest_cascade {
            write!(f, " ")?;
            for cascade in self.cascades.iter() {
                if let Some(card) = cascade.cards().get(row) {
                    write!(f, "{}", card)?;
                } else {
                    write!(f, "  ")?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Card, Cell, Deck, Foundation, Suit, Tableau};

    #[test]
    fn deal() {
        let tableau = Tableau::deal(Deck::fresh());
        tableau
            .cells
            .iter()
            .for_each(|cell| assert_eq!(&Cell::empty(), cell));
        tableau
            .foundations
            .iter()
            .for_each(|foundation| assert_eq!(&Foundation::empty(), foundation));
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
            tableau.cascades[0].cards().last(),
        );
    }

    #[test]
    fn is_won_empty() {
        assert!(Tableau::empty().is_won());
    }
}
