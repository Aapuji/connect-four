use std::io::{self, BufWriter, Write};

use crate::piece::{Piece, PieceDef};

#[derive(Debug, Clone)]
pub struct Board {
    width: usize,
    height: usize,
    pieces: Vec<Vec<Piece>>,
    selector: Selector,
}

impl Board {
    /// Creates a new `Board` given the `width` and `height`, clamped to be between 1 and 20.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width: width.clamp(1, 20),
            height: height.clamp(1, 20),
            pieces: vec![vec![Piece::Empty; width]; height],
            selector: Selector::new(1, width - 1),
        }
    }

    /// Displays the `Board` into the `writer`.
    ///
    /// Does not flush the `writer`.
    pub fn write<W: Write>(
        &self,
        writer: &mut BufWriter<W>,
        player: Piece,
        piece_def: PieceDef,
    ) -> io::Result<()> {
        // Selector line
        self.selector.write(writer, player, piece_def)?;

        // Board items
        for i in 0..self.height {
            for j in 0..self.width {
                writer.write(b"| ")?;
                self.pieces[i][j].write(writer, piece_def)?;
                writer.write(b" ")?;
            }

            writer.write(b"|\n")?;
        }

        // Bottom
        for _ in 0..self.width {
            writer.write(b"+---")?;
        }
        writer.write(b"+")?;

        Ok(())
    }

    pub fn selector_shl(&mut self) {
        self.selector.shl();
    }

    pub fn selector_shr(&mut self) {
        self.selector.shr();
    }

    pub fn selector_reset(&mut self) {
        self.selector.pos = 0;
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

/// A struct that corresponds to the selector row above the board.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Selector {
    pos: usize,
    max: usize,
}

impl Selector {
    /// Creates a new `Selector`. The `max` is the maximum possible column value that the sleector
    /// can have.
    pub fn new(pos: usize, max: usize) -> Self {
        Self { pos, max }
    }

    /// Shifts the selector left.
    pub fn shl(&mut self) {
        self.pos = self.pos.max(1) - 1;
    }

    /// Shifts the selector right.
    pub fn shr(&mut self) {
        self.pos = self.pos.min(self.max - 1) + 1;
    }

    pub fn write<W: Write>(
        &self,
        writer: &mut BufWriter<W>,
        player: Piece,
        piece_def: PieceDef,
    ) -> io::Result<()> {
        for j in 0..=self.max {
            if j == self.pos {
                writer.write("\x1b[38;2;65;72;104m\u{2039}\x1b[m(".as_bytes())?;
                player.write(writer, piece_def)?;
                writer.write(")\x1b[38;2;65;72;104m\u{203a}\x1b[m".as_bytes())?;
            } else {
                writer.write(b"    ")?;
            }
        }

        writer.write(b"\n")?;

        Ok(())
    }
}
