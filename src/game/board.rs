//! # Board
//!
//! Module for the chess board representation and related functionality.
//! Board is represented as a 64-bit unsigned integer, where each bit corresponds to a square on the
//! chess board. The least significant bit (LSB) represents the square a1, and the most significant
//! bit (MSB) represents the square h8. A bit value of 1 indicates that the square is occupied,
//! while a bit value of 0 indicates that the square is empty.

use super::auxil::flip_horizontal;
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

    /// Creates a new board from an array of strings, where each string represents a row of the board.
    /// Characters in the strings represent pieces, with uppercase letters for white pieces and lowercase
    /// letters for black pieces. Empty squares are represented by a dot ('.').
    pub fn from_array(board: [String; 8]) -> Result<Self, String> {
        let mut new_board = Board {
            white: Player::new(),
            black: Player::new(),
            player: true,
        };

        for (i, row) in board.iter().enumerate() {
            for (j, square) in row.chars().enumerate() {
                let bit_index = (7 - i) * 8 + j;
                match square {
                    'P' => new_board.white.pawns |= 1 << bit_index,
                    'p' => new_board.black.pawns |= 1 << bit_index,
                    'R' => new_board.white.rooks |= 1 << bit_index,
                    'r' => new_board.black.rooks |= 1 << bit_index,
                    'N' => new_board.white.knights |= 1 << bit_index,
                    'n' => new_board.black.knights |= 1 << bit_index,
                    'B' => new_board.white.bishops |= 1 << bit_index,
                    'b' => new_board.black.bishops |= 1 << bit_index,
                    'Q' => new_board.white.queens |= 1 << bit_index,
                    'q' => new_board.black.queens |= 1 << bit_index,
                    'K' => new_board.white.king |= 1 << bit_index,
                    'k' => new_board.black.king |= 1 << bit_index,
                    '.' => {}
                    _ => return Err(format!("Invalid character '{}' at ({}, {})", square, i, j)),
                }
            }
        }

        Ok(new_board)
    }

    /// Places black and white pieces on the board in their initial positions.
    pub fn place_pieces(&mut self) {
        self.white = Player::default();
        self.black = Player::default();
        self.player = true;

        // Flip the black pieces to their correct positions on the board.
        self.black.pawns = flip_horizontal(self.black.pawns);
        self.black.rooks = flip_horizontal(self.black.rooks);
        self.black.knights = flip_horizontal(self.black.knights);
        self.black.bishops = flip_horizontal(self.black.bishops);
        self.black.queens = flip_horizontal(self.black.queens);
        self.black.king = flip_horizontal(self.black.king);
    }
}

impl fmt::Display for Board {
    /// Formats the board for display, showing the pieces in their current positions.
    /// Characters are used to represent pieces, with uppercase letters for white pieces and lowercase
    /// letters for black pieces. Empty squares are represented by a dot ('.').
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in (0..8).rev() {
            for j in 0..8 {
                let bit_index = i * 8 + j;
                let square = if (self.white.pawns >> bit_index) & 1 == 1 {
                    'P'
                } else if (self.black.pawns >> bit_index) & 1 == 1 {
                    'p'
                } else if (self.white.rooks >> bit_index) & 1 == 1 {
                    'R'
                } else if (self.black.rooks >> bit_index) & 1 == 1 {
                    'r'
                } else if (self.white.knights >> bit_index) & 1 == 1 {
                    'N'
                } else if (self.black.knights >> bit_index) & 1 == 1 {
                    'n'
                } else if (self.white.bishops >> bit_index) & 1 == 1 {
                    'B'
                } else if (self.black.bishops >> bit_index) & 1 == 1 {
                    'b'
                } else if (self.white.queens >> bit_index) & 1 == 1 {
                    'Q'
                } else if (self.black.queens >> bit_index) & 1 == 1 {
                    'q'
                } else if (self.white.king >> bit_index) & 1 == 1 {
                    'K'
                } else if (self.black.king >> bit_index) & 1 == 1 {
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

        assert_eq!(board.white.pawns, 0x000000000000FF00);
        assert_eq!(board.black.pawns, 0x00FF000000000000);
        assert_eq!(board.white.rooks, 0x0000000000000081);
        assert_eq!(board.black.rooks, 0x8100000000000000);
        assert_eq!(board.white.knights, 0x0000000000000042);
        assert_eq!(board.black.knights, 0x4200000000000000);
        assert_eq!(board.white.bishops, 0x0000000000000024);
        assert_eq!(board.black.bishops, 0x2400000000000000);
        assert_eq!(board.white.queens, 0x0000000000000008);
        assert_eq!(board.black.queens, 0x0800000000000000);
        assert_eq!(board.white.king, 0x0000000000000010);
        assert_eq!(board.black.king, 0x1000000000000000);
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
        let mut board = Board {
            white: Player::new(),
            black: Player::new(),
            player: true,
        };
        board.place_pieces();

        assert_eq!(board.white.pawns, 0x000000000000FF00);
        assert_eq!(board.black.pawns, 0x00FF000000000000);
        assert_eq!(board.white.rooks, 0x0000000000000081);
        assert_eq!(board.black.rooks, 0x8100000000000000);
        assert_eq!(board.white.knights, 0x0000000000000042);
        assert_eq!(board.black.knights, 0x4200000000000000);
        assert_eq!(board.white.bishops, 0x0000000000000024);
        assert_eq!(board.black.bishops, 0x2400000000000000);
        assert_eq!(board.white.queens, 0x0000000000000010);
        assert_eq!(board.black.queens, 0x1000000000000000);
        assert_eq!(board.white.king, 0x0000000000000008);
        assert_eq!(board.black.king, 0x0800000000000000);
    }
}
