//! # Board
//!
//! Module for the chess board representation and related functionality.
//! Board is represented as a 64-bit unsigned integer, where each bit corresponds to a square on the
//! chess board. The least significant bit (LSB) represents the square a1, and the most significant
//! bit (MSB) represents the square h8. A bit value of 1 indicates that the square is occupied,
//! while a bit value of 0 indicates that the square is empty.

/// Player struct for holding the player pieces and related methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Player {
    pawns: u64,
    rooks: u64,
    knights: u64,
    bishops: u64,
    queens: u64,
    king: u64,
}

impl Default for Player {
    /// Places the pieces on the board on their initial positions.
    fn default() -> Self {
        Self {
            pawns: 0x000000000000FF00,
            rooks: 0x0000000000000081,
            knights: 0x0000000000000042,
            bishops: 0x0000000000000024,
            queens: 0x0000000000000010,
            king: 0x0000000000000008,
        }
    }
}

/// Main struct for holding the board state and related methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board {
    white: Player,
    black: Player,
    player: bool,
}

impl Board {
    fn place_pieces(&mut self) {}
}
