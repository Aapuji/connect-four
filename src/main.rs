use std::io::{self, BufWriter, Write};

use board::Board;
use crossterm::{cursor, execute};
use piece::PieceDef;

mod board;
mod piece;

fn main() -> io::Result<()> {
    let mut stdout = BufWriter::new(io::stdout());

    // Board
    let board = Board::new(7, 6);
    let piece_def = PieceDef::new("\x1b[31m●\x1b[m", "\x1b[33m●\x1b[m", " ");

    execute!(stdout, cursor::Hide)?;

    board.write(&mut stdout, piece_def)?;

    stdout.flush()?;

    execute!(stdout, cursor::Show)?;
    Ok(())
}
