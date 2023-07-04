use super::bitboard::BitBoard;

/// Represents a square on the game board. Can be retrieved as an index from
/// 0 to 64, as a bitboard mask, or simply used as an enum variant.
#[repr(u8)]
#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    pub fn index(self) -> u8 { self as u8 }
    pub fn bitboard(self) -> BitBoard { 1u64 << self.index() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_index_is_correct() {
        use Square::*;
        assert_eq!(A1.index(), 0);
        assert_eq!(B1.index(), 1);
        assert_eq!(A2.index(), 8);
        assert_eq!(B2.index(), 9);
        assert_eq!(H8.index(), 63);
    }

    #[test]
    fn square_bitboard_is_correct() {
        use Square::*;
        assert_eq!(A1.bitboard(), 0x00_00_00_00_00_00_00_01);
        assert_eq!(B1.bitboard(), 0x00_00_00_00_00_00_00_02);
        assert_eq!(E1.bitboard(), 0x00_00_00_00_00_00_00_10);
        assert_eq!(A2.bitboard(), 0x00_00_00_00_00_00_01_00);
        assert_eq!(H8.bitboard(), 0x80_00_00_00_00_00_00_00);
    }
}
