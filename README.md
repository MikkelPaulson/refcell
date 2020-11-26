# RefCell
A Freecell implementation in Rust.

## How to play

There are two binaries, one with a GUI created using
[Druid](https://github.com/linebender/druid/), the other a command-line
application. They both sit on top of game logic implemented at the data layer.

Running both of them requires Rust and Cargo to be installed on your local
environment.

### GUI

The GUI is incomplete and very, very broken.

    cargo run --bin gui

### CLI

The CLI works, although it doesn't provide convenience features like
automatically moving multiple cards at once. You will need a terminal capable of
displaying Unicode characters.

    cargo run --bin cli

Type the character for the source position followed by the destination position.
For instance, to move from the third column to the first free cell, type "3a".

The game will automatically end when there are no cards of higher rank on top
of cards of lower rank. You can end it prematurely by pressing ^C.
