use std::io::{self, BufWriter, Write};

use crate::piece::{Cell, Piece, PieceDef};

#[derive(Debug, Clone)]
pub struct Board {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
    selector: Selector,
}

impl Board {
    /// Creates a new `Board` given the `width` and `height`, clamped to be between 1 and 20.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width: width.clamp(1, 20),
            height: height.clamp(1, 20),
            cells: vec![vec![Cell(None); width]; height],
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
                self.cells[i][j].write(writer, piece_def)?;
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

    /// Resets cursor back to start, allowing for it to overwrite the previous board.
    pub fn reset_board<W: Write>(&self, writer: &mut BufWriter<W>) -> io::Result<()> {
        writer.write(format!("\x1b[{}A", self.get_entire_height()).as_bytes())?;

        Ok(())
    }

    /// Drops the given piece at the selector index.
    pub fn drop_piece(&mut self, piece: Piece) -> io::Result<()> {
        let mut y = self.height;

        while y > 0 && self.cells[self.height - y][self.selector.pos].is_empty() {
            y -= 1;
        }

        self.cells[self.height - y - 1][self.selector.pos].0 = Some(piece);

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

    /// Gets the "entire" height, including any extra new lines or other lines that are written
    /// around the board.
    pub fn get_entire_height(&self) -> usize {
        self.height + 3
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
