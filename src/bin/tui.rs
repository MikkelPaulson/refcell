use refcell::data::*;
use std::io;
use std::io::prelude::*;
use std::thread::sleep;
use std::time::Duration;
use termion::raw::IntoRawMode;
use termion::{cursor, input, screen};

fn main() -> io::Result<()> {
    let mut tableau = Tableau::deal(Deck::shuffled());

    let mut terminal =
        input::MouseTerminal::from(screen::AlternateScreen::from(io::stdout().into_raw_mode()?));
    let card = Card::new(Rank::Four, Suit::Clubs);

    render_card(&mut terminal, &card, 3)?;

    sleep(Duration::from_secs(5));

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

fn render_card(terminal: &mut impl Write, card: &Card, size: u8) -> io::Result<()> {
    write!(
        terminal,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )?;

    terminal.flush()?;

    Ok(())
}
