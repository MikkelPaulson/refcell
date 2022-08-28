use refcell::data::*;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::iter;
use std::thread::sleep;
use std::time::Duration;
use termion::raw::IntoRawMode;
use termion::{cursor, input, screen};

fn main() -> io::Result<()> {
    //let mut tableau = Tableau::deal(Deck::shuffled());

    let mut terminal =
        input::MouseTerminal::from(screen::AlternateScreen::from(io::stdout().into_raw_mode()?));

    for i in 0..=3 {
        let card = Rank::try_from(i)
            .ok()
            .map(|rank| Card::new(rank, Suit::Hearts));
        write!(terminal, "{}", termion::clear::All)?;

        {
            let mut col = 1;
            for (width, height) in [(3, 2), (5, 3), (7, 5), (9, 7)] {
                render_card(&mut terminal, card.as_ref(), col, 1, width, height)?;
                col += width + 1;
            }
        }

        {
            let mut col = 1;
            for width in 3..11 {
                let mut row = 10;
                for height in 2..9 {
                    render_card(&mut terminal, card.as_ref(), col, row, width, height)?;
                    row += height + 1;
                }
                col += width + 1;
            }
        }

        terminal.flush()?;
        sleep(Duration::from_secs(1));
    }

    /*
    writeln!(terminal, "{}", tableau).unwrap();

    while !tableau.is_won() {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("I/O error");

        match buffer.parse::<Action>() {
            Ok(action) => {
                if let Err(msg) = tableau.action(action) {
                    writeln!(terminal, "{}", msg).unwrap();
                }
            }
            Err(msg) => writeln!(terminal, "{}", msg).unwrap(),
        }
        writeln!(terminal, "{}", tableau).unwrap();
    }

    writeln!(terminal, "You win!").unwrap();
    */

    Ok(())
}

enum UiScale {
    Tiny,
    Small,
    Medium,
    Large,
}

impl UiScale {
    //fn from_terminal_size(size: (u16, u16)) -> Self {}

    fn get_card_size(&self) -> (u16, u16) {
        match self {
            UiScale::Tiny => (3, 2),
            UiScale::Small => (5, 3),
            UiScale::Medium => (7, 5),
            UiScale::Large => (9, 7),
        }
    }
}

struct CardView {
    size: (u16, u16),
    card: Option<Card>,
}

impl CardView {
    fn new(card: Option<&Card>, size: (u16, u16)) -> Self {
        Self {
            size,
            card: card.cloned(),
        }
    }

    fn char_at(&self, pos: (u16, u16)) -> char {
        if let Some(card) = self.card {
            let (suit, rank) = (card.get_suit(), card.get_rank());

            match self.card_char_at(pos) {
                CardChar::NumberLeft | CardChar::NumberOverflowLeft if rank == Rank::Ten => '1',
                CardChar::NumberRight | CardChar::NumberOverflowRight if rank == Rank::Ten => '0',
                CardChar::NumberLeft | CardChar::NumberRight => {
                    rank.as_str().chars().next().unwrap()
                }
                CardChar::Suit => suit.as_char(),
                CardChar::NumberOverflowLeft | CardChar::NumberOverflowRight | CardChar::Blank => {
                    ' '
                }
            }
        } else {
            let (width, height) = self.size;

            match pos {
                (0, 0) => '┏',
                (0, row) if row + 1 == height => '┗',
                (col, 0) if col + 1 == width => '┓',
                (col, row) if col + 1 == width && row + 1 == height => '┛',
                (col, _) if col == 0 || col + 1 == width => '┃',
                (_, row) if row == 0 || row + 1 == height => '━',
                _ => ' ',
            }
        }
    }

    fn card_char_at(&self, pos: (u16, u16)) -> CardChar {
        let card = if let Some(card) = self.card {
            card
        } else {
            return CardChar::Blank;
        };
        let (width, height) = self.size;

        match pos {
            (0, 0) => CardChar::NumberLeft,
            (1, 0) => CardChar::NumberOverflowRight,
            (col, 0) if col == width - 1 => CardChar::Suit,
            (0, row) if row == height - 1 => CardChar::Suit,
            _ => {
                if pos == (width - 2, height - 1) {
                    CardChar::NumberOverflowLeft
                } else if pos == (width - 1, height - 1) {
                    CardChar::NumberRight
                } else if width >= 6 && height >= 5 {
                    let (left, center_left, center, center_right, right) = (
                        1,
                        width / 3,
                        (width - 1) / 2,
                        (width * 2 - 1) / 3,
                        width - 2,
                    );
                    let (top, middle, bottom) = (1, (height - 1) / 2, height - 2);
                    let (col, row) = pos;

                    if match card.get_rank() {
                        Rank::Ace => col == center && row == middle,
                        Rank::Two => col == center && [top, bottom].contains(&row),
                        Rank::Three => col == center && [top, middle, bottom].contains(&row),
                        Rank::Four => [left, right].contains(&col) && [top, bottom].contains(&row),
                        Rank::Five => {
                            ([left, right].contains(&col) && [top, bottom].contains(&row))
                                || (col == center && row == middle)
                        }
                        Rank::Six => {
                            [left, center, right].contains(&col) && [top, bottom].contains(&row)
                        }
                        Rank::Seven => {
                            ([left, center, right].contains(&col) && [top, bottom].contains(&row))
                                || (col == center && row == middle)
                        }
                        Rank::Eight => {
                            [left, center_left, center_right, right].contains(&col)
                                && [top, bottom].contains(&row)
                        }
                        Rank::Nine => {
                            ([left, center_left, center_right, right].contains(&col)
                                && [top, bottom].contains(&row))
                                || (col == center && row == middle)
                        }
                        Rank::Ten => {
                            ([left, center_left, center_right, right].contains(&col)
                                && [top, bottom].contains(&row))
                                || ([center_left, center_right].contains(&col) && row == middle)
                        }
                        Rank::Jack => {
                            (row == top && (left..=right).contains(&col))
                                || (col == center && (top..=bottom).contains(&row))
                                || (row == bottom && (left..center).contains(&col))
                        }
                        Rank::Queen => {
                            ((left + 1..right).contains(&col) && row == top)
                                || ((left + 1..=right).contains(&col) && row == bottom)
                                || ((top + 1..bottom).contains(&row) && col == left)
                                || ((top + 1..=bottom).contains(&row) && col == right)
                        }
                        Rank::King => {
                            (col == left && (top..=bottom).contains(&row))
                                || ((left..right).contains(&col) && row == middle)
                                || (col == right && (top..=bottom).contains(&row) && row != middle)
                        }
                    } {
                        CardChar::Suit
                    } else {
                        CardChar::Blank
                    }
                } else {
                    CardChar::Blank
                }
            }
        }
    }

    fn line_at(&self, row: u16) -> String {
        if let Some(card) = self.card {
            if card.get_suit().is_red() {
                "\x1b[91;7m"
            } else {
                "\x1b[39;7m"
            }
        } else {
            ""
        }
        .chars()
        .chain((0..self.size.0).map(|col| self.char_at((col, row))))
        .chain("\x1b[0m".chars())
        .collect()
    }
}

#[derive(Clone, Copy, Debug)]
enum CardChar {
    NumberLeft,
    NumberRight,
    NumberOverflowLeft,
    NumberOverflowRight,
    Suit,
    Blank,
}

fn render_card(
    terminal: &mut impl Write,
    card: Option<&Card>,
    col: u16,
    row: u16,
    width: u16,
    height: u16,
) -> io::Result<()> {
    let card_view = CardView::new(card, (width, height));

    (0..height).try_for_each(|i| {
        write!(
            terminal,
            "{goto}{line}",
            goto = termion::cursor::Goto(col, i as u16 + row),
            line = card_view.line_at(i)
        )
    })
}
