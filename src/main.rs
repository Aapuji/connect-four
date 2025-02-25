use std::{
    io::{self, BufWriter, Stdout, Write},
    time::Duration,
};

use board::Board;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute, queue, terminal,
};
use piece::{Piece, PieceDef, Player};

mod board;
mod game;
mod piece;

/// Waits until player either enters 'q', 'Q', Left, Right, or Enter.
fn get_input() -> io::Result<KeyCode> {
    loop {
        let event = event::read()?;

        match event {
            Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) => {
                if let KeyCode::Left
                | KeyCode::Right
                | KeyCode::Char('q')
                | KeyCode::Char('Q')
                | KeyCode::Enter = code
                {
                    return Ok(code);
                }
            }
            _ => (),
        }
    }
}

fn main() -> io::Result<()> {
    let mut stdout = BufWriter::new(io::stdout());

    // Board
    let mut board = Board::new(7, 6);
    let piece_def = PieceDef::new(
        "\x1b[38;2;247;118;142m●\x1b[m",
        "\x1b[38;2;224;174;104m●\x1b[m",
        " ",
    );
    const DROP_RATE_DELAY: usize = 10;

    // Game
    let mut game_over = false;
    let mut player = Player::P1;

    // Capture the Enter when running the program
    if event::poll(Duration::from_secs(0))? {
        event::read()?;
    }

    execute!(stdout, cursor::Hide)?;

    while !game_over {
        board.write(&mut stdout, Piece::Reg(player), piece_def)?;
        stdout.write(b"\n\n")?;
        stdout.flush()?;

        let key = get_input()?;

        match key {
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                stdout.write(b"Thanks for playing :)")?;
                game_over = true;
                break;
            }
            KeyCode::Enter => {
                // Drop thingy
                board.drop_piece(Piece::Reg(player))?;

                player = player.next();
                board.selector_reset();
                board.reset_board(&mut stdout)?;
            }
            KeyCode::Left => {
                board.selector_shl();
                board.reset_board(&mut stdout)?;
            }
            KeyCode::Right => {
                board.selector_shr();
                board.reset_board(&mut stdout)?;
            }
            _ => (),
        }
    }

    stdout.flush()?;

    execute!(stdout, cursor::Show)?;
    Ok(())
}
