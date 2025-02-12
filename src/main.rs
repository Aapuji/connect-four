use std::io::{self, BufWriter, Write};

use board::Board;
use crossterm::{cursor, execute};
use piece::{Piece, PieceDef};

mod board;
mod piece;

fn main() -> io::Result<()> {
    let mut stdout = BufWriter::new(io::stdout());

    // Board
    let board = Board::new(7, 6);
    let piece_def = PieceDef::new(
        "\x1b[38;2;247;118;142m●\x1b[m",
        "\x1b[38;2;224;174;104m●\x1b[m",
        " ",
    );

    execute!(stdout, cursor::Hide)?;

    board.write(&mut stdout, Piece::P2, piece_def)?;

    stdout.flush()?;

    execute!(stdout, cursor::Show)?;
    Ok(())
}
