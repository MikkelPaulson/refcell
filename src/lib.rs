use std::convert::TryInto;
use std::fmt;
use std::iter;

pub use action::{Action, FromCoordinate, ToCoordinate};
pub use cascade::Cascade;
pub use cell::Cell;
pub use deck::{Card, Deck, Rank, Suit};
pub use foundation::Foundation;
pub use single::Single;

mod action;
mod cascade;
mod cell;
mod deck;
mod foundation;
mod single;

#[derive(Clone, Debug)]
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
            .for_each(|(card, i)| tableau.cascades[i].push(card));

        tableau
    }

    pub fn action(&mut self, action: Action) -> Result<(), &'static str> {
        if let Action {
            from: FromCoordinate::Cascade(n_from),
            to: ToCoordinate::Cascade(n_to),
        } = action
        {
            let (n_from, n_to) = (n_from as usize, n_to as usize);
            let (from, to) = (&self.cascades[n_from], &self.cascades[n_to]);

            if from.is_empty() {
                return Err("That space is empty.");
            }

            if let Some(expected_rank) = to
                .cards()
                .last()
                .and_then(|card| card.get_rank().try_decrement())
            {
                let max_stack_size = {
                    let num_empty_cascades = self
                        .cascades
                        .iter()
                        .enumerate()
                        .filter(|(i, c)| ![n_from, n_to].contains(i) && c.is_empty())
                        .count();

                    let num_empty_cells = self.cells.iter().filter(|cell| cell.is_empty()).count();

                    ((num_empty_cells + 1) * (num_empty_cascades + 1)).min(from.len())
                };

                for i in 1..=max_stack_size {
                    if from.cards()[from.len() - i].get_rank() == expected_rank {
                        if let Some(stack) = self.cascades[n_from].try_pop_stack(i) {
                            return if let Err((stack, message)) =
                                self.cascades[n_to].try_push_stack(stack)
                            {
                                self.cascades[n_from].push_stack(stack);
                                Err(message)
                            } else {
                                Ok(())
                            };
                        }

                        break;
                    }
                }
            }
        }

        let card = match action.from {
            FromCoordinate::Cascade(n) => self.cascades[n as usize].pop(),
            FromCoordinate::Cell(n) => self.cells[n as usize].take(),
        }
        .ok_or("That space is empty.")?;

        if let Err((card, message)) = match action.to {
            ToCoordinate::Cascade(n) => self.cascades[n as usize].try_push(card),
            ToCoordinate::Cell(n) => self.cells[n as usize].try_push(card),
            ToCoordinate::Foundation(n) => self.foundations[n as usize].try_push(card),
        } {
            match action.from {
                FromCoordinate::Cascade(n) => self.cascades[n as usize].push(card),
                FromCoordinate::Cell(n) => self.cells[n as usize].try_push(card).unwrap(),
            }

            Err(message)
        } else {
            Ok(())
        }
    }

    pub fn is_won(&self) -> bool {
        self.cascades.iter().all(|cascade| cascade.is_sequential())
    }
}

impl fmt::Display for Tableau {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // cells and foundations
        {
            writeln!(f, " A   B   C   D   W   X   Y   Z")?;
            let top_row: Vec<String> = self
                .cells
                .iter()
                .map(|cell| cell.peek())
                .chain(self.foundations.iter().map(|foundation| foundation.peek()))
                .map(|card| match card {
                    Some(card) => card.to_string(),
                    None => "\x1b[2;7m   \x1b[0m\n\x1b[2;7m   \x1b[0m".to_string(),
                })
                .collect();

            top_row
                .iter()
                .try_for_each(|s| write!(f, "{} ", s.lines().next().unwrap()))?;
            writeln!(f)?;

            top_row
                .iter()
                .try_for_each(|s| write!(f, "{} ", s.lines().last().unwrap()))?;
            writeln!(f)?;
        }

        writeln!(f)?;

        // cascades
        {
            writeln!(f, " 1   2   3   4   5   6   7   8")?;

            let longest_cascade = self
                .cascades
                .iter()
                .map(|cascade| cascade.len())
                .max()
                .unwrap_or(0);

            for row in 0..longest_cascade + 1 {
                self.cascades.iter().try_for_each(|cascade| {
                    cascade
                        .cards()
                        .get(row)
                        .map(|card| write!(f, "{} ", card.to_string().lines().next().unwrap()))
                        .or_else(|| {
                            row.checked_sub(1)
                                .and_then(|prev_row| cascade.cards().get(prev_row))
                                .map(|card| {
                                    write!(f, "{} ", card.to_string().lines().last().unwrap())
                                })
                        })
                        .unwrap_or_else(|| write!(f, "    "))
                })?;

                writeln!(f)?;
            }
        }

        Ok(())
    }
}

/*
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

        let foundation_suits: HashMap<Suit, u8> = self
            .foundations
            .iter()
            .filter_map(|f| f.get_suit().map(|suit| (suit, f.get_rank())))
            .collect();

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
                    if foundation_suits
                        .get(&card.get_suit())
                        .cloned()
                        .unwrap_or_default()
                        + 1
                        == card.get_rank()
                    {
                        write!(f, "\x1b[7m{}", card)?;
                    } else {
                        write!(f, "{}", card)?;
                    }
                } else {
                    write!(f, "  ")?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
*/

#[cfg(test)]
mod tests {
    use super::{
        Action, Card, Cell, Deck, Foundation, FromCoordinate, Rank, Single, Suit, Tableau,
        ToCoordinate,
    };

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
                ((i / 4) + 1).try_into().unwrap(),
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

        assert!(tableau.is_won(), "{:?}", tableau);

        // Just to be sure: we have aces on top, right?
        assert_eq!(
            Some(&Card::new(Rank::Ace, Suit::Clubs)),
            tableau.cascades[0].cards().last(),
        );
    }

    #[test]
    fn is_won_empty() {
        assert!(Tableau::empty().is_won());
    }

    #[test]
    fn action_legal_to_foundation() {
        let mut tableau = Tableau::empty();
        tableau.cells[0]
            .try_push(Card::new(Rank::Ace, Suit::Clubs))
            .unwrap();
        tableau.cascades[0]
            .try_push(Card::new(Rank::Two, Suit::Clubs))
            .unwrap();

        assert_eq!(
            Ok(()),
            tableau.action(Action {
                from: FromCoordinate::Cell(0),
                to: ToCoordinate::Foundation(0),
            }),
        );
        assert_eq!(
            Ok(()),
            tableau.action(Action {
                from: FromCoordinate::Cascade(0),
                to: ToCoordinate::Foundation(0),
            }),
        );

        assert!(tableau.cells[0].is_empty());
        assert_eq!(0, tableau.cascades[0].len());
        assert_eq!(
            Some(&Card::new(Rank::Two, Suit::Clubs)),
            tableau.foundations[0].peek(),
        );
    }

    #[test]
    fn action_legal_to_cascade() {
        let mut tableau = Tableau::empty();
        tableau.cells[0]
            .try_push(Card::new(Rank::King, Suit::Clubs))
            .unwrap();
        tableau.cascades[0]
            .try_push(Card::new(Rank::Queen, Suit::Hearts))
            .unwrap();

        assert_eq!(
            Ok(()),
            tableau.action(Action {
                from: FromCoordinate::Cell(0),
                to: ToCoordinate::Cascade(1),
            }),
        );

        assert_eq!(
            Ok(()),
            tableau.action(Action {
                from: FromCoordinate::Cascade(0),
                to: ToCoordinate::Cascade(1),
            }),
        );

        assert!(tableau.cells[0].is_empty());
        assert_eq!(0, tableau.cascades[0].len());
        assert_eq!(2, tableau.cascades[1].len());
    }

    #[test]
    fn action_legal_to_cell() {
        let mut tableau = Tableau::empty();
        tableau.cells[0]
            .try_push(Card::new(Rank::Ace, Suit::Hearts))
            .unwrap();
        tableau.cascades[0]
            .try_push(Card::new(Rank::Ace, Suit::Spades))
            .unwrap();

        assert_eq!(
            Ok(()),
            tableau.action(Action {
                from: FromCoordinate::Cell(0),
                to: ToCoordinate::Cell(1),
            }),
        );

        assert_eq!(
            Ok(()),
            tableau.action(Action {
                from: FromCoordinate::Cascade(0),
                to: ToCoordinate::Cell(2),
            }),
        );

        assert!(tableau.cells[0].is_empty());
        assert_eq!(0, tableau.cascades[0].len());
        assert_eq!(
            Some(&Card::new(Rank::Ace, Suit::Hearts)),
            tableau.cells[1].peek()
        );
        assert_eq!(
            Some(&Card::new(Rank::Ace, Suit::Spades)),
            tableau.cells[2].peek()
        );
    }

    #[test]
    fn action_illegal() {
        let mut tableau = Tableau::empty();
        tableau.cascades[0].push(Card::new(Rank::King, Suit::Hearts));
        tableau.cells[0]
            .try_push(Card::new(Rank::Queen, Suit::Hearts))
            .unwrap();
        tableau.cells[1]
            .try_push(Card::new(Rank::Jack, Suit::Hearts))
            .unwrap();

        assert_eq!(
            Err("That card cannot go on that cascade."),
            tableau.action(Action {
                from: FromCoordinate::Cell(0),
                to: ToCoordinate::Cascade(0),
            }),
        );

        assert_eq!(
            Err("A card is already present on that cell."),
            tableau.action(Action {
                from: FromCoordinate::Cascade(0),
                to: ToCoordinate::Cell(0),
            }),
        );

        assert_eq!(
            Err("That card is not valid on that foundation."),
            tableau.action(Action {
                from: FromCoordinate::Cell(1),
                to: ToCoordinate::Foundation(0),
            }),
        );

        assert_eq!(
            Some(&Card::new(Rank::King, Suit::Hearts)),
            tableau.cascades[0].cards().last(),
        );

        assert_eq!(
            Some(&Card::new(Rank::Queen, Suit::Hearts)),
            tableau.cells[0].peek()
        );
        assert_eq!(
            Some(&Card::new(Rank::Jack, Suit::Hearts)),
            tableau.cells[1].peek()
        );
    }

    #[test]
    fn action_illegal_empty() {
        let mut tableau = Tableau::empty();

        assert_eq!(
            Err("That space is empty."),
            tableau.action(Action {
                from: FromCoordinate::Cell(0),
                to: ToCoordinate::Cell(1)
            }),
        );

        assert_eq!(
            Err("That space is empty."),
            tableau.action(Action {
                from: FromCoordinate::Cascade(0),
                to: ToCoordinate::Cell(2)
            }),
        );
    }
}
