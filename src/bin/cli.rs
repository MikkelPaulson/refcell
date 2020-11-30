use refcell::data::*;

use std::io;

fn main() {
    let mut tableau = Tableau::deal(Deck::shuffled());

    println!("{}", tableau);
    println!("Type the character for the source position followed by the destination position.\nFor instance, to move from the third column to the first free cell, type \"3a\".");

    while !tableau.is_won() {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("I/O error");

        match buffer.parse::<Action>() {
            Ok(action) => {
                if let Some(card) = match action.from {
                    Coordinate::Cascade(n) => tableau.cascades[n as usize].pop(),
                    Coordinate::Cell(n) => tableau.cells[n as usize].take(),
                    Coordinate::Foundation(_) => unreachable!(),
                } {
                    if let Err((card, message)) = match action.to {
                        Coordinate::Cascade(n) => tableau.cascades[n as usize].push(card),
                        Coordinate::Cell(n) => tableau.cells[n as usize].push(card),
                        Coordinate::Foundation(n) => tableau.foundations[n as usize].push(card),
                    } {
                        match action.from {
                            Coordinate::Cascade(n) => {
                                tableau.cascades[n as usize].push_unchecked(card)
                            }
                            Coordinate::Cell(n) => tableau.cells[n as usize].push(card).unwrap(),
                            Coordinate::Foundation(_) => unreachable!(),
                        }

                        println!("{}", message);
                    }
                } else {
                    println!("That space is empty.");
                }
            }
            Err(message) => println!("{}", message),
        }
        println!("{}", tableau);
    }

    println!("You win!");
}
