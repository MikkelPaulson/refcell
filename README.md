# RefCell
A Freecell implementation in Rust.

## How to play

There are two binaries, a rich command-line app that supports a mouse interface,
as well as a simple stdin/stdout app.  They both sit on top of game logic
implemented at the data layer.

Running both of them requires Rust and Cargo to be installed on your local
environment.

### CLI

The CLI works, although it doesn't provide convenience features like
automatically moving multiple cards at once. I suggest enlarging the font size
of your terminal to make everything easier to read, since the rendering is quite
small.

    cargo run --bin cli

Type the character for the source position followed by the destination position.
For instance, to move from the third column to the first free cell, type "3a".

The game will automatically end when there are no cards of higher rank on top
of cards of lower rank. You can end it prematurely by pressing ^C.

### TUI

The terminal UI is incomplete. It'll also run at the command line, but will
be aware of terminal size and support mouse input.

    cargo run --bin tui

No interactivity is implemented at this point.
