//! # Auxilliary module
//!
//! This module contains various helper functions and types that are used throughout the chess
//! engine. It is not intended to be used directly by users of the library, but rather to support
//! the implementation of the core functionality.

/// Creates a lookup table for reversing the bits in a byte. This table is used to efficiently reverse
/// the bits in a 64-bit unsigned integer by reversing the bits in each byte and then
const fn make_reverse_bits_table() -> [u8; 256] {
    let mut table = [0u8; 256];
    let mut i = 0;

    while i < 256 {
        table[i] = (i as u8).reverse_bits();
        i += 1;
    }

    table
}
const REVERSE_BITS: [u8; 256] = make_reverse_bits_table();

/// Reverses the bits in each byte of a 64-bit unsigned integer.
/// Uses algorithm from Hacker's Delight, 2nd edition, by Henry S. Warren, Jr.
fn reverse_bits_by_byte(x: u64) -> u64 {
    let mut x = x;
    let mut result = 0u64;

    for i in 0..8 {
        let byte = (x & 0xFF) as u8;
        result |= (REVERSE_BITS[byte as usize] as u64) << (i * 8);
        x >>= 8;
    }

    result
}

/// Transposes a 64-bit unsigned integer, treating it as an 8x8 matrix of bits. The LSB represents
/// the square a1, and the MSB represents the square h8.
fn transpose(x: u64) -> u64 {
    let mut x = x;
    let mut t = (x ^ (x >> 7)) & 0x00AA00AA00AA00AA;
    x ^= t ^ (t << 7);

    t = (x ^ (x >> 14)) & 0x0000CCCC0000CCCC;
    x ^= t ^ (t << 14);

    t = (x ^ (x >> 28)) & 0x00000000F0F0F0F0;
    x ^= t ^ (t << 28);

    x
}

/// Reverses the order of the bytes in a 64-bit unsigned integer. The LSB
/// becomes the MSB, and vice versa.
fn reverse_bytes(x: u64) -> u64 {
    let mut x = x;
    let mut result = 0u64;

    for i in 0..8 {
        let byte = (x & 0xFF) as u8;
        result |= (byte as u64) << ((7 - i) * 8);
        x >>= 8;
    }

    result
}

/// Rotates a 64-bit unsigned integer by 180 degrees, rotating the board.
/// This is done by reversing the bits in each byte and then reversing the order of the bytes.
pub fn rotate_by_180(x: u64) -> u64 {
    reverse_bytes(reverse_bits_by_byte(x))
}

/// Flips a 64-bit unsigned integer vertically, flipping the board upside down.
/// This is done by swapping the bits in each row of the board.
pub fn flip_vertical(x: u64) -> u64 {
    reverse_bits_by_byte(x)
}

pub fn flip_horizontal(x: u64) -> u64 {
    reverse_bytes(x)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reverse_bits_by_byte() {
        let input: u64 = 0x0123456789ABCDEF;
        let expected_output: u64 = 0x80C4A2E691D5B3F7;
        assert_eq!(reverse_bits_by_byte(input), expected_output);
    }

    #[test]
    fn test_reverse_bytes() {
        let input: u64 = 0x0123456789ABCDEF;
        let expected_output: u64 = 0xEFCDAB8967452301;
        assert_eq!(reverse_bytes(input), expected_output);
    }

    #[test]
    fn test_transpose() {
        let input: u64 = 0x0123456789ABCDEF;
        let expected_output: u64 = 0x0F3355000F3355FF;
        assert_eq!(transpose(input), expected_output);
    }

    #[test]
    fn test_rotate_by_180() {
        let input: u64 = 0x0123456789ABCDEF;
        let expected_output: u64 = 0xF7B3D591E6A2C480;
        assert_eq!(rotate_by_180(input), expected_output);
    }

    #[test]
    fn test_flip_vertical() {
        let input: u64 = 0x0123456789ABCDEF;
        let expected_output: u64 = 0x80C4A2E691D5B3F7;
        assert_eq!(flip_vertical(input), expected_output);
    }

    #[test]
    fn test_flip_horizontal() {
        let input: u64 = 0x0123456789ABCDEF;
        let expected_output: u64 = 0xEFCDAB8967452301;
        assert_eq!(flip_horizontal(input), expected_output);
    }
}
