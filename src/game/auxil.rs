//! # Auxilliary module
//!
//! This module contains various helper functions and types that are used throughout the chess
//! engine. It is not intended to be used directly by users of the library, but rather to support
//! the implementation of the core functionality.

/// Converts a board representation from an array of strings to a 64-bit unsigned integer.
/// The board is read from left to right, bottom to top.
pub fn board_to_uint(board: &[&str; 8]) -> Result<u64, String> {
    if board.len() != 8 {
        return Err(format!(
            "Invalid board length: {}. Board must have exactly 8 rows.",
            board.len()
        ));
    }

    let mut result: u64 = 0;

    for (i, row) in board.iter().enumerate() {
        for (j, c) in row.chars().enumerate() {
            match c {
                'X' => result |= 1 << ((7 - i) * 8 + j),
                '.' => (),
                _ => {
                    return Err(format!(
                        "Invalid character '{}' at position ({}, {}). Only 'X' and '.' are allowed.",
                        c, i, j
                    ));
                }
            }
        }
    }

    Ok(result)
}

/// Converts a 64-bit unsigned integer representation of a board to an array of strings.
/// The board is read from left to right, bottom to top.
pub fn uint_to_board(board: u64) -> [String; 8] {
    let mut result = std::array::from_fn(|_| String::new());

    for (i, row) in result.iter_mut().enumerate() {
        for j in 0..8 {
            let bit_index = (7 - i) * 8 + j;
            if (board & (1 << bit_index)) != 0 {
                row.push('X');
            } else {
                row.push('.');
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_board_to_uint() {
        let board = [
            "........", "........", "........", "........", "........", "........", "........",
            "XXXXXXXX",
        ];
        let result = super::board_to_uint(&board).unwrap();
        assert_eq!(result, 0x0000000000000FF);

        let board = [
            "...X....", "...X..XX", "..XX....", "X......X", "..X.....", "........", "........",
            ".......X",
        ];
        let result = super::board_to_uint(&board).unwrap();
        assert_eq!(
            result,
            0b0000100011001000000011001000000100000100000000000000000010000000
        );
    }

    #[test]
    fn test_uint_to_board() {
        let board = 0x0000000000000FF;
        let result = super::uint_to_board(board);
        assert_eq!(
            result,
            [
                "........", "........", "........", "........", "........", "........", "........",
                "XXXXXXXX"
            ]
        );

        let board = 0b0000100011001000000011001000000100000100000000000000000010000000;
        let result = super::uint_to_board(board);
        assert_eq!(
            result,
            [
                "...X....", "...X..XX", "..XX....", "X......X", "..X.....", "........", "........",
                ".......X"
            ]
        );
    }
}
