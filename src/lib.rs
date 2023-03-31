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

#[derive(Debug)]
pub struct Game {
    history: Vec<Tableau>,
}

#[derive(Clone, Debug)]
pub struct Tableau {
    pub cells: [Cell; 4],
    pub foundations: [Foundation; 4],
    pub cascades: [Cascade; 8],
}

impl Game {
    pub fn new(tableau: Tableau) -> Self {
        Self {
            history: vec![tableau],
        }
    }

    pub fn deal(mut deck: Deck) -> Self {
        let mut tableau = Tableau::empty();

        iter::from_fn(|| deck.pop())
            .zip((0..8).cycle())
            .for_each(|(card, i)| tableau.cascades[i].push(card));

        Self::new(tableau)
    }

    pub fn action(&mut self, action: Action) -> Result<(), &'static str> {
        match action {
            Action::Undo => {
                if self.history.len() > 1 {
                    self.history.pop();
                    Ok(())
                } else {
                    Err("You are already at the first move.")
                }
            }
            Action::MoveCard { from, to, count } => {
                let mut tableau = self.tableau().clone();

                if let (FromCoordinate::Cascade(n_from), ToCoordinate::Cascade(n_to)) = (from, to) {
                    let (n_from, n_to) = (n_from as usize, n_to as usize);
                    let (from_cascade, to_cascade) =
                        (&tableau.cascades[n_from], &tableau.cascades[n_to]);

                    if from_cascade.is_empty() {
                        return Err("That space is empty.");
                    }

                    let max_stack_size = {
                        let num_empty_cascades = tableau
                            .cascades
                            .iter()
                            .enumerate()
                            .filter(|(i, c)| ![n_from, n_to].contains(i) && c.is_empty())
                            .count();

                        let num_empty_cells =
                            tableau.cells.iter().filter(|cell| cell.is_empty()).count();

                        ((num_empty_cells + 1) * (num_empty_cascades + 1))
                            .min(from_cascade.len())
                            .min(
                                count
                                    .and_then(|i| u8::from(i).try_into().ok())
                                    .unwrap_or(usize::MAX),
                            )
                    };

                    if let Some(expected_rank) = to_cascade
                        .cards()
                        .last()
                        .and_then(|card| card.get_rank().try_decrement())
                    {
                        for i in 1..=max_stack_size {
                            if from_cascade.cards()[from_cascade.len() - i].get_rank()
                                == expected_rank
                            {
                                if let Some(stack) = tableau.cascades[n_from].try_pop_stack(i) {
                                    match tableau.cascades[n_to].try_push_stack(stack) {
                                        Ok(()) => {
                                            self.history.push(tableau);
                                            return Ok(());
                                        }
                                        Err((stack, message)) => {
                                            tableau.cascades[n_from].push_stack(stack);
                                            return Err(message);
                                        }
                                    }
                                }

                                break;
                            }
                        }
                    } else if let Some(count) = count {
                        if let Some(stack) = tableau.cascades[n_from]
                            .try_pop_stack(max_stack_size.min(u8::from(count).try_into().unwrap()))
                        {
                            match tableau.cascades[n_to].try_push_stack(stack) {
                                Ok(()) => {
                                    self.history.push(tableau);
                                    return Ok(());
                                }
                                Err((stack, message)) => {
                                    tableau.cascades[n_from].push_stack(stack);
                                    return Err(message);
                                }
                            }
                        }
                    }
                }

                let card = match from {
                    FromCoordinate::Cascade(n) => tableau.cascades[n as usize].pop(),
                    FromCoordinate::Cell(n) => tableau.cells[n as usize].take(),
                }
                .ok_or("That space is empty.")?;

                if let Err((card, message)) = match to {
                    ToCoordinate::Cascade(n) => tableau.cascades[n as usize].try_push(card),
                    ToCoordinate::Cell(n) => tableau.cells[n as usize].try_push(card),
                    ToCoordinate::Foundation(n) => tableau.foundations[n as usize].try_push(card),
                } {
                    match from {
                        FromCoordinate::Cascade(n) => tableau.cascades[n as usize].push(card),
                        FromCoordinate::Cell(n) => {
                            tableau.cells[n as usize].try_push(card).unwrap()
                        }
                    }

                    Err(message)
                } else {
                    self.history.push(tableau);
                    Ok(())
                }
            }
        }
    }

    pub fn is_won(&self) -> bool {
        self.tableau()
            .cascades
            .iter()
            .all(|cascade| cascade.is_sequential())
    }

    fn tableau(&self) -> &Tableau {
        self.history.last().unwrap()
    }
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
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.tableau())
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
    use super::*;

    #[test]
    fn deal() {
        let game = Game::deal(Deck::fresh());
        game.tableau()
            .cells
            .iter()
            .for_each(|cell| assert_eq!(&Cell::empty(), cell));
        game.tableau()
            .foundations
            .iter()
            .for_each(|foundation| assert_eq!(&Foundation::empty(), foundation));
        game.tableau().cascades[0..4]
            .iter()
            .for_each(|cascade| assert_eq!(7, cascade.len()));
        game.tableau().cascades[4..8]
            .iter()
            .for_each(|cascade| assert_eq!(6, cascade.len()));

        assert_eq!(
            52,
            game.tableau()
                .cascades
                .iter()
                .fold(0, |i, cascade| i + cascade.len()),
        );
    }

    #[test]
    fn is_not_won_fresh() {
        let game = Game::deal(Deck::fresh());
        assert_eq!(false, game.is_won());
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
        let game = Game::deal(deck);

        assert!(game.is_won(), "{:?}", game);

        // Just to be sure: we have aces on top, right?
        assert_eq!(
            Some(&Card::new(Rank::Ace, Suit::Clubs)),
            game.tableau().cascades[0].cards().last(),
        );
    }

    #[test]
    fn is_won_empty() {
        assert!(Game::new(Tableau::empty()).is_won());
    }

    #[test]
    fn action_legal_to_foundation() {
        let mut game = {
            let mut tableau = Tableau::empty();

            tableau.cells[0]
                .try_push(Card::new(Rank::Ace, Suit::Clubs))
                .unwrap();
            tableau.cascades[0]
                .try_push(Card::new(Rank::Two, Suit::Clubs))
                .unwrap();

            Game::new(tableau)
        };

        assert_eq!(
            Ok(()),
            game.action(Action::MoveCard {
                from: FromCoordinate::Cell(0),
                to: ToCoordinate::Foundation(0),
                count: None,
            }),
        );
        assert_eq!(
            Ok(()),
            game.action(Action::MoveCard {
                from: FromCoordinate::Cascade(0),
                to: ToCoordinate::Foundation(0),
                count: None,
            }),
        );

        assert!(game.tableau().cells[0].is_empty());
        assert_eq!(0, game.tableau().cascades[0].len());
        assert_eq!(
            Some(&Card::new(Rank::Two, Suit::Clubs)),
            game.tableau().foundations[0].peek(),
        );
    }

    #[test]
    fn action_legal_to_cascade() {
        let mut game = {
            let mut tableau = Tableau::empty();

            tableau.cells[0]
                .try_push(Card::new(Rank::King, Suit::Clubs))
                .unwrap();
            tableau.cascades[0]
                .try_push(Card::new(Rank::Queen, Suit::Hearts))
                .unwrap();

            Game::new(tableau)
        };

        assert_eq!(
            Ok(()),
            game.action(Action::MoveCard {
                from: FromCoordinate::Cell(0),
                to: ToCoordinate::Cascade(1),
                count: None,
            }),
        );

        assert_eq!(
            Ok(()),
            game.action(Action::MoveCard {
                from: FromCoordinate::Cascade(0),
                to: ToCoordinate::Cascade(1),
                count: None,
            }),
        );

        assert!(game.tableau().cells[0].is_empty());
        assert_eq!(0, game.tableau().cascades[0].len());
        assert_eq!(2, game.tableau().cascades[1].len());
    }

    #[test]
    fn action_legal_to_cell() {
        let mut game = {
            let mut tableau = Tableau::empty();

            tableau.cells[0]
                .try_push(Card::new(Rank::Ace, Suit::Hearts))
                .unwrap();
            tableau.cascades[0]
                .try_push(Card::new(Rank::Ace, Suit::Spades))
                .unwrap();

            Game::new(tableau)
        };

        assert_eq!(
            Ok(()),
            game.action(Action::MoveCard {
                from: FromCoordinate::Cell(0),
                to: ToCoordinate::Cell(1),
                count: None,
            }),
        );

        assert_eq!(
            Ok(()),
            game.action(Action::MoveCard {
                from: FromCoordinate::Cascade(0),
                to: ToCoordinate::Cell(2),
                count: None,
            }),
        );

        assert!(game.tableau().cells[0].is_empty());
        assert_eq!(0, game.tableau().cascades[0].len());
        assert_eq!(
            Some(&Card::new(Rank::Ace, Suit::Hearts)),
            game.tableau().cells[1].peek(),
        );
        assert_eq!(
            Some(&Card::new(Rank::Ace, Suit::Spades)),
            game.tableau().cells[2].peek()
        );
    }

    #[test]
    fn action_illegal() {
        let mut game = {
            let mut tableau = Tableau::empty();

            tableau.cascades[0].push(Card::new(Rank::King, Suit::Hearts));
            tableau.cells[0]
                .try_push(Card::new(Rank::Queen, Suit::Hearts))
                .unwrap();
            tableau.cells[1]
                .try_push(Card::new(Rank::Jack, Suit::Hearts))
                .unwrap();

            Game::new(tableau)
        };

        assert_eq!(
            Err("That card cannot go on that cascade."),
            game.action(Action::MoveCard {
                from: FromCoordinate::Cell(0),
                to: ToCoordinate::Cascade(0),
                count: None,
            }),
        );

        assert_eq!(
            Err("A card is already present on that cell."),
            game.action(Action::MoveCard {
                from: FromCoordinate::Cascade(0),
                to: ToCoordinate::Cell(0),
                count: None,
            }),
        );

        assert_eq!(
            Err("That card is not valid on that foundation."),
            game.action(Action::MoveCard {
                from: FromCoordinate::Cell(1),
                to: ToCoordinate::Foundation(0),
                count: None,
            }),
        );

        assert_eq!(
            Some(&Card::new(Rank::King, Suit::Hearts)),
            game.tableau().cascades[0].cards().last(),
        );

        assert_eq!(
            Some(&Card::new(Rank::Queen, Suit::Hearts)),
            game.tableau().cells[0].peek()
        );
        assert_eq!(
            Some(&Card::new(Rank::Jack, Suit::Hearts)),
            game.tableau().cells[1].peek()
        );
    }

    #[test]
    fn action_illegal_empty() {
        let mut game = Game::new(Tableau::empty());

        assert_eq!(
            Err("That space is empty."),
            game.action(Action::MoveCard {
                from: FromCoordinate::Cell(0),
                to: ToCoordinate::Cell(1),
                count: None,
            }),
        );

        assert_eq!(
            Err("That space is empty."),
            game.action(Action::MoveCard {
                from: FromCoordinate::Cascade(0),
                to: ToCoordinate::Cell(2),
                count: None,
            }),
        );
    }
}
