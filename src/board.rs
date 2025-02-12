use std::{
    io::{self, BufWriter, Write},
    ops,
};

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
            selector: Selector::new(0),
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
        self.selector.write(writer, player, piece_def, self.width)?;

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
        self.selector.shl(0);
    }

    pub fn selector_shr(&mut self) {
        self.selector.shr(self.width - 1);
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
}

impl Selector {
    pub fn new(pos: usize) -> Self {
        Self { pos }
    }

    /// Shifts the selector left, stopping at `min`, which should be the smallest column value that
    /// the selector could have.
    pub fn shl(&mut self, min: usize) {
        self.pos = self.pos.max(min + 1) - 1;
    }

    /// Shifts the selector right, stopping at `max`, which should be the biggest column value that
    /// the selector could have.
    ///
    /// Note: This means you will pass in `width - 1`.
    pub fn shr(&mut self, max: usize) {
        self.pos = self.pos.min(max - 1) + 1;
    }

    pub fn write<W: Write>(
        &self,
        writer: &mut BufWriter<W>,
        player: Piece,
        piece_def: PieceDef,
        width: usize,
    ) -> io::Result<()> {
        for j in 0..width {
            if j == self.pos {
                writer.write(b" (")?;
                player.write(writer, piece_def)?;
                writer.write(b")")?;
            } else {
                writer.write(b"    ")?;
            }
        }

        writer.write(b" \n")?;

        Ok(())
    }
}
