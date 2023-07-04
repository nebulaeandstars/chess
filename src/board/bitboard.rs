pub trait IsBitBoard: Copy {
    fn inc_rank_by(self, amount: i8) -> Self;
    fn inc_rank(self) -> Self;
    fn dec_rank(self) -> Self;

    fn inc_file(self) -> Self;
    fn dec_file(self) -> Self;
}

pub type BitBoard = u64;
impl IsBitBoard for BitBoard {
    fn inc_rank_by(self, amount: i8) -> Self { self << (8 * amount) }
    fn inc_rank(self) -> Self { self << 8 }
    fn dec_rank(self) -> Self { self >> 8 }

    fn inc_file(self) -> Self { self << 1 & FILE_A }
    fn dec_file(self) -> Self { self >> 1 & FILE_H }
}

pub const FULL: BitBoard = 0xff_ff_ff_ff_ff_ff_ff_ff;
pub const EMPTY: BitBoard = 0x00_00_00_00_00_00_00_00;

pub const LIGHT_SQUARES: BitBoard = 0x55_aa_55_aa_55_aa_55_aa;
pub const DARK_SQUARES: BitBoard = 0xaa_55_aa_55_aa_55_aa_55;

pub const RANK_8: BitBoard = 0xff_00_00_00_00_00_00_00;
pub const RANK_7: BitBoard = 0x00_ff_00_00_00_00_00_00;
pub const RANK_6: BitBoard = 0x00_00_ff_00_00_00_00_00;
pub const RANK_5: BitBoard = 0x00_00_00_ff_00_00_00_00;
pub const RANK_4: BitBoard = 0x00_00_00_00_ff_00_00_00;
pub const RANK_3: BitBoard = 0x00_00_00_00_00_ff_00_00;
pub const RANK_2: BitBoard = 0x00_00_00_00_00_00_ff_00;
pub const RANK_1: BitBoard = 0x00_00_00_00_00_00_00_ff;

pub const FILE_A: BitBoard = 0x01_01_01_01_01_01_01_01;
pub const FILE_B: BitBoard = 0x02_02_02_02_02_02_02_02;
pub const FILE_C: BitBoard = 0x04_04_04_04_04_04_04_04;
pub const FILE_D: BitBoard = 0x08_08_08_08_08_08_08_08;
pub const FILE_E: BitBoard = 0x10_10_10_10_10_10_10_10;
pub const FILE_F: BitBoard = 0x20_20_20_20_20_20_20_20;
pub const FILE_G: BitBoard = 0x40_40_40_40_40_40_40_40;
pub const FILE_H: BitBoard = 0x80_80_80_80_80_80_80_80;

pub const BLACK_HALF: BitBoard = 0xff_ff_ff_ff_00_00_00_00;
pub const WHITE_HALF: BitBoard = 0x00_00_00_00_ff_ff_ff_ff;
pub const QUEENSIDE: BitBoard = 0x0f_0f_0f_0f_0f_0f_0f_0f;
pub const KINGSIDE: BitBoard = 0xf0_f0_f0_f0_f0_f0_f0_f0;

pub const CENTER_4: BitBoard = 0x00_00_00_18_18_00_00_00;
pub const CENTER_16: BitBoard = 0x00_00_3c_3c_3c_3c_00_00;
pub const EDGE: BitBoard = 0xff_81_81_81_81_81_81_ff;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn union_of_halves_is_whole() {
        assert_eq!(LIGHT_SQUARES | DARK_SQUARES, FULL);
        assert_eq!(LIGHT_SQUARES & DARK_SQUARES, EMPTY);
        assert_eq!(BLACK_HALF | WHITE_HALF, FULL);
        assert_eq!(BLACK_HALF & WHITE_HALF, EMPTY);
        assert_eq!(KINGSIDE | QUEENSIDE, FULL);
        assert_eq!(KINGSIDE & QUEENSIDE, EMPTY);
    }
}
