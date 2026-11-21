//! # Board
//!
//! Module for the chess board representation and related functionality.
//! Board is represented as a 64-bit unsigned integer, where each bit corresponds to a square on the
//! chess board. The least significant bit (LSB) represents the square a1, and the most significant
//! bit (MSB) represents the square h8. A bit value of 1 indicates that the square is occupied,
//! while a bit value of 0 indicates that the square is empty.

use std::fmt;

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

/// Main struct for holding the board state and related methods. The player whose turn it is always
/// sees the board from their perspective, with their pieces on the bottom and the opponent's
/// pieces on the top.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board {
    white: Player,
    black: Player,
    player: bool,
}

impl Board {
    /// Switches the perspective of the board, swapping the white and black pieces and changing the
    /// current player.
    fn switch_perspective(&self) -> Self {
        let mut new_board = *self;

        new_board.white = self.black;
        new_board.black = self.white;
        new_board.player = !self.player;

        new_board
    }

    pub fn from_array(board: [String; 8]) -> Result<Self, String> {
        let mut new_board = Board {
            white: Player::default(),
            black: Player::default(),
            player: true,
        };

        for (i, row) in board.iter().enumerate() {
            for (j, square) in row.chars().enumerate() {
                let bit_index = (7 - i) * 8 + j;
                match square {
                    'X' => new_board.white.pawns |= 1 << bit_index,
                    'O' => new_board.black.pawns |= 1 << bit_index,
                    '.' => {}
                    _ => return Err(format!("Invalid character '{}' at ({}, {})", square, i, j)),
                }
            }
        }

        Ok(new_board)
    }

    /// Places black and white pieces on the board in their initial positions.
    pub fn place_pieces(&mut self) {}
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {}
}
#[cfg(test)]
mod tests {}
