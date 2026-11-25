//! # Board
//!
//! Module for the chess board representation and related functionality.
//! Board is represented as a 64-bit unsigned integer, where each bit corresponds to a square on the
//! chess board. The least significant bit (LSB) represents the square a1, and the most significant
//! bit (MSB) represents the square h8. A bit value of 1 indicates that the square is occupied,
//! while a bit value of 0 indicates that the square is empty.

use crate::game::auxil::flip_horizontal;
use std::{
    fmt,
    ops::{Index, IndexMut},
};

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

impl Player {
    fn new() -> Self {
        Self {
            pawns: 0,
            rooks: 0,
            knights: 0,
            bishops: 0,
            queens: 0,
            king: 0,
        }
    }
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
    // 0 for pawn, 1 for rook, 2 for knight, 3 for bishop, 4 for queen, 5 for king
    pieces: [[u64; 6]; 2],
    player: u8, // 0 for white, 1 for black
}

/// Default board state, all pieces 0.
impl Default for Board {
    fn default() -> Self {
        Self {
            pieces: [[0u64; 6]; 2],
            player: 0,
        }
    }
}

/// Index trait, to access pieces by `Board[player]` instead of `Board.pieces[player]`.
impl Index<usize> for Board {
    type Output = [u64; 6];
    fn index(&self, index: usize) -> &Self::Output {
        &self.pieces[index]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.pieces[index]
    }
}

impl Board {
    /// Switches the perspective of the board, swapping the white and black pieces and changing the
    /// current player.
    fn switch_perspective(&self) -> Self {
        let mut new_board = *self;

        (new_board[0], new_board[1]) = (new_board[1], new_board[0]);
        new_board.player = !new_board.player;

        new_board
    }

    /// Creates a new board from an array of strings, where each string represents a row of the board.
    /// Characters in the strings represent pieces, with uppercase letters for white pieces and lowercase
    /// letters for black pieces. Empty squares are represented by a dot ('.').
    pub fn from_array(board: [String; 8]) -> Result<Self, String> {
        let mut new_board = Board::default();

        for (i, row) in board.iter().rev().enumerate() {
            for (j, square) in row.chars().rev().enumerate() {
                let bit_index = 8 * i + j;
                match square {
                    'P' => new_board[0][0] |= 1 << bit_index,
                    'p' => new_board[1][0] |= 1 << bit_index,
                    'R' => new_board[0][1] |= 1 << bit_index,
                    'r' => new_board[1][1] |= 1 << bit_index,
                    'N' => new_board[0][2] |= 1 << bit_index,
                    'n' => new_board[1][2] |= 1 << bit_index,
                    'B' => new_board[0][3] |= 1 << bit_index,
                    'b' => new_board[1][3] |= 1 << bit_index,
                    'Q' => new_board[0][4] |= 1 << bit_index,
                    'q' => new_board[1][4] |= 1 << bit_index,
                    'K' => new_board[0][5] |= 1 << bit_index,
                    'k' => new_board[1][5] |= 1 << bit_index,
                    '.' => {}
                    _ => return Err(format!("Invalid character '{}' at ({}, {})", square, i, j)),
                }
            }
        }

        Ok(new_board)
    }

    /// Places black and white pieces on the board in their initial positions.
    pub fn place_pieces(&mut self) {
        for player in 0..=1 {
            self[player][0] = 0x000000000000FF00;
            self[player][1] = 0x0000000000000081;
            self[player][2] = 0x0000000000000042;
            self[player][3] = 0x0000000000000024;
            self[player][4] = 0x0000000000000010;
            self[player][5] = 0x0000000000000008;
        }

        // Flip the black pieces to their correct positions on the board.
        let mut black_pieces = [0u64; 6];
        for (index, pieces) in self[1].iter().enumerate() {
            black_pieces[index] = flip_horizontal(pieces);
        }
        self[1] = black_pieces;
    }
}

impl fmt::Display for Board {
    /// Formats the board for display, showing the pieces in their current positions.
    /// Characters are used to represent pieces, with uppercase letters for white pieces and lowercase
    /// letters for black pieces. Empty squares are represented by a dot ('.').
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in (0..8).rev() {
            for j in (0..8).rev() {
                let bit_index = i * 8 + j;
                let square = if (self[0][0] >> bit_index) & 1 == 1 {
                    'P'
                } else if (self[1][0] >> bit_index) & 1 == 1 {
                    'p'
                } else if (self[0][1] >> bit_index) & 1 == 1 {
                    'R'
                } else if (self[1][1] >> bit_index) & 1 == 1 {
                    'r'
                } else if (self[0][2] >> bit_index) & 1 == 1 {
                    'N'
                } else if (self[1][2] >> bit_index) & 1 == 1 {
                    'n'
                } else if (self[0][3] >> bit_index) & 1 == 1 {
                    'B'
                } else if (self[1][3] >> bit_index) & 1 == 1 {
                    'b'
                } else if (self[0][4] >> bit_index) & 1 == 1 {
                    'Q'
                } else if (self[1][4] >> bit_index) & 1 == 1 {
                    'q'
                } else if (self[0][5] >> bit_index) & 1 == 1 {
                    'K'
                } else if (self[1][5] >> bit_index) & 1 == 1 {
                    'k'
                } else {
                    '.'
                };
                write!(f, "{}", square)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_array() {
        let board_array = [
            "rnbqkbnr".to_string(),
            "pppppppp".to_string(),
            "........".to_string(),
            "........".to_string(),
            "........".to_string(),
            "........".to_string(),
            "PPPPPPPP".to_string(),
            "RNBQKBNR".to_string(),
        ];

        let board = Board::from_array(board_array).unwrap();

        assert_eq!(board[0][0], 0x000000000000FF00);
        assert_eq!(board[1][0], 0x00FF000000000000);
        assert_eq!(board[0][1], 0x0000000000000081);
        assert_eq!(board[1][1], 0x8100000000000000);
        assert_eq!(board[0][2], 0x0000000000000042);
        assert_eq!(board[1][2], 0x4200000000000000);
        assert_eq!(board[0][3], 0x0000000000000024);
        assert_eq!(board[1][3], 0x2400000000000000);
        assert_eq!(board[0][4], 0x0000000000000010);
        assert_eq!(board[1][4], 0x1000000000000000);
        assert_eq!(board[0][5], 0x0000000000000008);
        assert_eq!(board[1][5], 0x0800000000000000);
    }

    #[test]
    fn test_display() {
        let board_array = [
            "rnbqkbnr".to_string(),
            "pppppppp".to_string(),
            "........".to_string(),
            "........".to_string(),
            "........".to_string(),
            "........".to_string(),
            "PPPPPPPP".to_string(),
            "RNBQKBNR".to_string(),
        ];

        let board = Board::from_array(board_array).unwrap();
        let expected_display = "\
rnbqkbnr
pppppppp
........
........
........
........
PPPPPPPP
RNBQKBNR
";
        assert_eq!(format!("{}", board), expected_display);
    }

    #[test]
    fn test_place_pieces() {
        let mut board = Board::default();
        board.place_pieces();

        assert_eq!(board[0][0], 0x000000000000FF00);
        assert_eq!(board[1][0], 0x00FF000000000000);
        assert_eq!(board[0][1], 0x0000000000000081);
        assert_eq!(board[1][1], 0x8100000000000000);
        assert_eq!(board[0][2], 0x0000000000000042);
        assert_eq!(board[1][2], 0x4200000000000000);
        assert_eq!(board[0][3], 0x0000000000000024);
        assert_eq!(board[1][3], 0x2400000000000000);
        assert_eq!(board[0][4], 0x0000000000000010);
        assert_eq!(board[1][4], 0x1000000000000000);
        assert_eq!(board[0][5], 0x0000000000000008);
        assert_eq!(board[1][5], 0x0800000000000000);
    }
}
