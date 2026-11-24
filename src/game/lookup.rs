//! # Lookup module
//!
//! Module containing lookup tables and functions for bit manipulation and board representation.

use std::collections::HashMap;

/// Generates a direction lookup table from each board position.
/// For example, (10, (1, -1)) has all ones starting at the square c2 in direction (1, -1), that is,
/// north-east.
const fn generate_direction_lookup() {
    let mut lookup_table: HashMap<(u64, (i8, i8)), u64> = HashMap::new();
}

/// Generates a lookup table for valid pawn moves but not captures from each square on the chessboard.
const fn pawn_valid_move_lookup() -> [u64; 64] {
    let mut lookup = [0u64; 64];
    let mut i = 0;
    while i < 64 {
        let rank = i / 8;
        let file = 7 - (i % 8);
        let mut moves = 0u64;

        // Pawns can move one square forward if not on the last rank
        if rank < 7 {
            moves |= 1u64 << ((rank + 1) * 8 + file);
        }

        // Pawns can move two squares forward from their initial position.
        if rank == 1 {
            moves |= 1u64 << ((rank + 2) * 8 + file);
        }

        lookup[i] = moves;
        i += 1;
    }

    lookup
}

/// Generates a lookup table for valid pawn captures from each square on the chessboard.
const fn pawn_valid_capture_lookup() -> [u64; 64] {
    let mut lookup = [0u64; 64];
    let mut i = 0;
    while i < 64 {
        let rank = i / 8;
        let file = 7 - (i % 8);
        let mut captures = 0u64;

        // Pawns can capture diagonally to the left if not on the first file and not on the last rank.
        if rank < 7 && file > 0 {
            captures |= 1u64 << ((rank + 1) * 8 + file - 1);
        }

        // Pawns can capture diagonally to the right if not on the last file and not on the last rank.
        if rank < 7 && file < 7 {
            captures |= 1u64 << ((rank + 1) * 8 + file + 1);
        }

        lookup[i] = captures;
        i += 1;
    }

    lookup
}

/// Generates a lookup table for valid rook moves from each square on the chessboard.
const fn rook_valid_lookup() -> [u64; 64] {
    let mut lookup = [0u64; 64];
    let mut i = 0;
    while i < 64 {
        let rank = i / 8;
        let file = 7 - (i % 8);
        let mut moves = 0u64;

        // Horizontal moves
        let mut f = 0;
        while f < 8 {
            if f != file {
                moves |= 1u64 << (rank * 8 + f);
            }
            f += 1;
        }

        // Vertical moves
        let mut r = 0;
        while r < 8 {
            if r != rank {
                moves |= 1u64 << (r * 8 + file);
            }
            r += 1;
        }

        lookup[i] = moves;
        i += 1;
    }

    lookup
}

/// Generates a lookup table for valid knight moves from each square on the chessboard.
const fn knight_valid_lookup() -> [u64; 64] {
    let mut lookup = [0u64; 64];
    let mut i = 0;
    while i < 64 {
        let rank = i / 8;
        let file = 7 - (i % 8);
        let mut moves = 0u64;

        // Possible knight moves
        let knight_moves = [
            (2, 1),
            (2, -1),
            (-2, 1),
            (-2, -1),
            (1, 2),
            (1, -2),
            (-1, 2),
            (-1, -2),
        ];

        let mut j = 0;
        while j < 8 {
            let (dr, df) = knight_moves[j];
            let new_rank = rank as isize + dr;
            let new_file = file as isize + df;

            if new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
                moves |= 1u64 << (new_rank * 8 + new_file);
            }
            j += 1;
        }

        lookup[i] = moves;
        i += 1;
    }

    lookup
}

/// Generates a lookup table for valid bishop moves from each square on the chessboard.
const fn bishop_valid_lookup() -> [u64; 64] {
    let mut lookup = [0u64; 64];
    let mut i = 0;

    while i < 64 {
        let rank = i / 8;
        let file = 7 - (i % 8);
        let mut moves = 0u64;

        // Diagonal moves
        let mut dr = -7;
        while dr <= 7 {
            if dr == 0 {
                dr += 1;
                continue;
            }
            let new_rank = rank as isize + dr;
            let new_file1 = file as isize + dr;
            let new_file2 = file as isize - dr;

            if new_rank >= 0 && new_rank < 8 {
                if new_file1 >= 0 && new_file1 < 8 {
                    moves |= 1u64 << (new_rank * 8 + new_file1);
                }
                if new_file2 >= 0 && new_file2 < 8 {
                    moves |= 1u64 << (new_rank * 8 + new_file2);
                }
            }

            dr += 1;
        }

        lookup[i] = moves;
        i += 1;
    }

    lookup
}

/// Generates a lookup table for valid queen moves from each square on the chessboard.
const fn queen_valid_lookup() -> [u64; 64] {
    // Calculate queen moves for each square as OR of rook and bishop.
    let mut lookup = [0u64; 64];

    let mut i = 0;
    while i < 64 {
        let rook_moves = ROOK_VALID_LOOKUP[i];
        let bishop_moves = BISHOP_VALID_LOOKUP[i];

        lookup[i] = rook_moves | bishop_moves;
        i += 1;
    }

    lookup
}
/// Generates a lookup table for valid king moves from each square on the chessboard.
const fn king_valid_lookup() -> [u64; 64] {
    let mut lookup = [0u64; 64];

    let mut i = 0;
    while i < 64 {
        let mut moves = 0u64;

        let rank = i / 8;
        let file = 7 - (i % 8);

        let mut dr = -1;
        while dr <= 1 {
            let mut df = -1;
            while df <= 1 {
                let new_rank = rank as isize + dr;
                let new_file = file as isize + df;

                if (new_rank == rank as isize && new_file == file as isize) {
                    // Skip piece position.
                    df += 1;
                    continue;
                }

                if (new_rank >= 0 && new_rank <= 7) && (new_file >= 0 && new_file <= 7) {
                    moves |= 1u64 << (new_rank * 8 + new_file);
                }

                df += 1;
            }

            dr += 1;
        }

        lookup[i] = moves;
        i += 1;
    }

    lookup
}

pub const PAWN_VALID_MOVES_LOOKUP: [u64; 64] = pawn_valid_move_lookup();
pub const PAWN_VALID_CAPTURES_LOOKUP: [u64; 64] = pawn_valid_capture_lookup();
pub const ROOK_VALID_LOOKUP: [u64; 64] = rook_valid_lookup();
pub const KNIGHT_VALID_LOOKUP: [u64; 64] = knight_valid_lookup();
pub const BISHOP_VALID_LOOKUP: [u64; 64] = bishop_valid_lookup();
pub const QUEEN_VALID_LOOKUP: [u64; 64] = queen_valid_lookup();
pub const KING_VALID_LOOKUP: [u64; 64] = king_valid_lookup();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pawn_valid_move_lookup() {
        // Test a few squares for pawn moves
        assert_eq!(PAWN_VALID_MOVES_LOOKUP[8], 0x0000000080800000); // Square a2
        assert_eq!(PAWN_VALID_MOVES_LOOKUP[9], 0x0000000040400000); // Square b2
        assert_eq!(PAWN_VALID_MOVES_LOOKUP[16], 0x0000000080000000); // Square a3
    }

    #[test]
    fn test_pawn_valid_capture_lookup() {
        // Test a few squares for pawn captures
        assert_eq!(PAWN_VALID_CAPTURES_LOOKUP[8], 0x0000000000400000); // Square a2
        assert_eq!(PAWN_VALID_CAPTURES_LOOKUP[9], 0x0000000000A00000); // Square b2
        assert_eq!(PAWN_VALID_CAPTURES_LOOKUP[16], 0x0000000040000000); // Square a3
    }

    #[test]
    fn test_rook_valid_lookup() {
        // Test a few squares for rook moves
        assert_eq!(
            ROOK_VALID_LOOKUP[0],
            0x8080808080808000 | 0x000000000000007F
        ); // Square a1
        assert_eq!(
            ROOK_VALID_LOOKUP[7],
            0x0101010101010100 | 0x00000000000000FE
        ); // Square h1
        assert_eq!(
            ROOK_VALID_LOOKUP[50],
            0x2000202020202020 | 0x00DF000000000000
        ); // Square c7
    }

    #[test]
    fn test_knight_valid_lookup() {
        // Test a few squares for knight moves
        assert_eq!(
            KNIGHT_VALID_LOOKUP[1],
            0x0000000000A00000 | 0x0000000000001000
        ); // Square b1
        assert_eq!(
            KNIGHT_VALID_LOOKUP[52],
            0x2200000000000000 | 0x0000220000000000 | 0x0000001400000000
        ); // Square e7
        assert_eq!(KNIGHT_VALID_LOOKUP[18], 0x0000005088008850); // Square c3
    }

    #[test]
    fn test_bishop_valid_lookup() {
        // Test a few squares for bishop moves
        assert_eq!(BISHOP_VALID_LOOKUP[0], 0x0102040810204000); // Square a1
        assert_eq!(BISHOP_VALID_LOOKUP[7], 0x8040201008040200); // Square h1
        assert_eq!(BISHOP_VALID_LOOKUP[36], 0x4122140014224180); // Square e5
    }

    #[test]
    fn test_queen_valid_lookup() {
        // Test a few squares for queen moves.
        assert_eq!(QUEEN_VALID_LOOKUP[0], 0x8182848890A0C07F); // Square a1
        assert_eq!(QUEEN_VALID_LOOKUP[36], 0x492A1CF71C2A4988); // Square e5
        assert_eq!(QUEEN_VALID_LOOKUP[47], 0x0503FE0305091121); // Square h6
    }

    #[test]
    fn test_king_valid_lookup() {
        // Test a few squares for king moves.
        assert_eq!(KING_VALID_LOOKUP[0], 0x000000000000C040); // Square a1
        assert_eq!(KING_VALID_LOOKUP[36], 0x00001C141C000000); // Square e5
        assert_eq!(KING_VALID_LOOKUP[47], 0x0003020300000000); // Square h6
    }
}
