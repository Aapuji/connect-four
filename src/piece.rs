use std::io::{self, BufWriter, Write};

/// An enum representing the possible value of a cell. Either it is player-1, player-2, or empty.
/// When rendering, the actual rendered values for the variants can be chosen.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    P1,
    P2,
    Empty,
}

impl Piece {
    pub fn is_empty(self) -> bool {
        match self {
            Self::P1 | Self::P2 => false,
            Self::Empty => true,
        }
    }

    pub fn write<W: Write>(self, writer: &mut BufWriter<W>, piece_def: PieceDef) -> io::Result<()> {
        writer.write(
            match self {
                Self::P1 => piece_def.p1,
                Self::P2 => piece_def.p2,
                Self::Empty => piece_def.empty,
            }
            .as_bytes(),
        )?;

        Ok(())
    }
}

/// Represents the actual string represenations of the `Piece` variants.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PieceDef {
    p1: &'static str,
    p2: &'static str,
    empty: &'static str,
}

impl PieceDef {
    pub fn new(p1: &'static str, p2: &'static str, empty: &'static str) -> Self {
        Self { p1, p2, empty }
    }
}
