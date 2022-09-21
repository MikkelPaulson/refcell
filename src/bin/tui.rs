use refcell::data::*;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::iter;
use std::thread::sleep;
use std::time::Duration;
use termion::raw::IntoRawMode;
use termion::{color, cursor, input, screen};

fn main() -> io::Result<()> {
    let mut tableau = Tableau::deal(Deck::shuffled());

    let _hide_cursor = termion::cursor::HideCursor::from(io::stdout());
    let mut terminal =
        input::MouseTerminal::from(screen::AlternateScreen::from(io::stdout().into_raw_mode()?));

    clear(&mut terminal)?;

    let scale = UiScale::from_terminal_size(termion::terminal_size()?);

    render_tableau(&mut terminal, &tableau, scale)?;
    terminal.flush()?;
    sleep(Duration::from_secs(10));

    Ok(())
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum UiScale {
    Tiny,
    Small,
    Medium,
    Large,
}

impl UiScale {
    fn from_terminal_size(size: (u16, u16)) -> Self {
        let scales = [
            UiScale::Large,
            UiScale::Medium,
            UiScale::Small,
            UiScale::Tiny,
        ];

        let scale_x = scales
            .iter()
            .find(|scale| size.0 >= scale.get_card_size().0 * 8 + 7)
            .unwrap_or(&UiScale::Tiny);

        let scale_y = scales
            .iter()
            .find(|scale| size.1 >= scale.get_card_size().1 * 2 + 10)
            .unwrap_or(&UiScale::Tiny);

        *scale_x.min(scale_y)
    }

    fn get_column_spacing(&self) -> u16 {
        self.get_card_width() + if self == &UiScale::Large { 2 } else { 1 }
    }

    fn get_row_spacing(&self) -> u16 {
        self.get_card_height() + 1
    }

    fn get_card_width(&self) -> u16 {
        self.get_card_size().0
    }

    fn get_card_height(&self) -> u16 {
        self.get_card_size().1
    }

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
    scale: UiScale,
    card: Option<Card>,
    highlighted: bool,
}

impl CardView {
    fn new(card: Option<&Card>, scale: UiScale) -> Self {
        Self {
            scale,
            card: card.cloned(),
            highlighted: false,
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
        } else if pos.0 == self.scale.get_card_width() / 2
            && pos.1 == self.scale.get_card_height() / 2
        {
            '✖'
        } else {
            ' '
        }
    }

    fn card_char_at(&self, pos: (u16, u16)) -> CardChar {
        let card = if let Some(card) = self.card {
            card
        } else {
            return CardChar::Blank;
        };
        let (width, height) = self.scale.get_card_size();

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
        let line: String = (0..self.scale.get_card_width())
            .map(|col| self.char_at((col, row)))
            .collect();

        format!(
            "{}{}{}{}{}",
            if self.card.is_none() {
                color::Fg(color::AnsiValue::grayscale(8))
            } else if self.highlighted {
                color::Fg(color::AnsiValue::grayscale(24))
            } else {
                color::Fg(color::AnsiValue::grayscale(18))
            },
            match self.card.map(|card| card.get_suit().is_red()) {
                Some(true) => color::Bg(color::AnsiValue::rgb(4, 0, 0)),
                Some(false) => color::Bg(color::AnsiValue::grayscale(6)),
                None => color::Bg(color::AnsiValue::grayscale(18)),
            },
            line,
            color::Fg(color::Reset),
            color::Bg(color::Reset),
        )
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

fn clear(terminal: &mut impl Write) -> io::Result<()> {
    write!(terminal, "{}", termion::clear::All)?;

    let (width, height) = termion::terminal_size()?;

    for row in 1..=height {
        write!(terminal, "{}\x1b[48;5;22m", cursor::Goto(1, row))?;
        for col in 1..=width {
            write!(terminal, " ")?;
        }
    }

    Ok(())
}

fn render_tableau(terminal: &mut impl Write, tableau: &Tableau, scale: UiScale) -> io::Result<()> {
    tableau
        .cells
        .iter()
        .map(|c| c.peek())
        .chain(tableau.foundations.iter().map(|f| f.peek()))
        .enumerate()
        .try_for_each(|(col, card)| {
            render_card(
                terminal,
                card,
                col as u16 * scale.get_column_spacing() + 1,
                1,
                scale,
            )
        })?;

    for (col, cascade) in tableau.cascades.iter().enumerate() {
        for (row, card) in cascade.cards().iter().enumerate() {
            render_card(
                terminal,
                Some(card),
                col as u16 * scale.get_column_spacing() + 1,
                row as u16 + scale.get_row_spacing() + 1,
                scale,
            )?;
        }
    }

    Ok(())
}

fn render_card(
    terminal: &mut impl Write,
    card: Option<&Card>,
    col: u16,
    row: u16,
    scale: UiScale,
) -> io::Result<()> {
    let card_view = CardView::new(card, scale);

    (0..scale.get_card_height()).try_for_each(|i| {
        write!(
            terminal,
            "{goto}{line}",
            goto = termion::cursor::Goto(col, i as u16 + row),
            line = card_view.line_at(i)
        )
    })
}
