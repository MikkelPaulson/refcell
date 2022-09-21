use refcell::*;

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
                if let Err(msg) = tableau.action(action) {
                    println!("{}", msg);
                }
            }
            Err(msg) => println!("{}", msg),
        }
        println!("{}", tableau);
    }

    println!("You win!");
}
