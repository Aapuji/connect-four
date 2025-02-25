use std::io::{self, BufWriter, Write};

/// A struct representing the possible values of a cell on the board.
///
/// It can either be a `Piece` or Empty (None)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell(pub Option<Piece>);

impl Cell {
    pub fn write<W: Write>(
        &self,
        writer: &mut BufWriter<W>,
        piece_def: PieceDef,
    ) -> io::Result<()> {
        match self.0 {
            Some(piece) => piece.write(writer, piece_def)?,
            None => {
                writer.write(piece_def.empty.as_bytes())?;
                ()
            }
        };

        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }
}

/// Represents the player.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    P1,
    P2,
}

impl Player {
    pub fn next(self) -> Self {
        match self {
            Self::P1 => Self::P2,
            Self::P2 => Self::P1,
        }
    }
}

/// An enum representing the possible value of a cell. Either it is player-1, player-2. Empty is
/// represented by None, while players are Some(Piece).
/// When rendering, the actual rendered values for the variants can be chosen.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    Reg(Player),
}

impl Piece {
    pub fn write<W: Write>(
        &self,
        writer: &mut BufWriter<W>,
        piece_def: PieceDef,
    ) -> io::Result<()> {
        writer.write(
            match self {
                Self::Reg(Player::P1) => piece_def.p1,
                Self::Reg(Player::P2) => piece_def.p2,
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
