//! # Lookup module
//!
//! Module containing lookup tables and functions for bit manipulation and board representation.

pub const RANK_MASKS: [u64; 8] = [
    0x00000000000000FF,
    0x000000000000FF00,
    0x0000000000FF0000,
    0x00000000FF000000,
    0x000000FF00000000,
    0x0000FF0000000000,
    0x00FF000000000000,
    0xFF00000000000000,
];
pub const FILE_MASKS: [u64; 8] = [
    0x0101010101010101,
    0x0202020202020202,
    0x0404040404040404,
    0x0808080808080808,
    0x1010101010101010,
    0x2020202020202020,
    0x4040404040404040,
    0x8080808080808080,
];

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

pub const PAWN_VALID_MOVES_LOOKUP: [u64; 64] = pawn_valid_move_lookup();
pub const PAWN_VALID_CAPTURES_LOOKUP: [u64; 64] = pawn_valid_capture_lookup();
pub const ROOK_VALID_LOOKUP: [u64; 64] = rook_valid_lookup();

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
}
