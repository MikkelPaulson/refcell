use super::{Card, Deck, Suit};
use std::convert::TryInto;
use std::iter;

pub use cascade::Cascade;
pub use cell::Cell;
pub use foundation::Foundation;

mod cascade;
mod cell;
mod foundation;

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
