use crate::types::*;

pub struct Piece {
    pub piece_type: PieceType,
    pub square:     Square,
}

/// A type of chess piece.
///
/// We're using WhiteKing, BlackRook, etc. instead of King(PieceColor) and the
/// like, as the former can be represented as a u8, while the latter requires
/// more space as it technically contains multiple values.
#[repr(u8)]
pub enum PieceType {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

#[repr(u8)]
pub enum PieceColor {
    White,
    Black,
}

impl PieceType {
    pub fn color(&self) -> PieceColor {
        use PieceColor::*;
        use PieceType::*;
        match self {
            WhitePawn => White,
            WhiteKnight => White,
            WhiteBishop => White,
            WhiteRook => White,
            WhiteQueen => White,
            WhiteKing => White,
            BlackPawn => Black,
            BlackKnight => Black,
            BlackBishop => Black,
            BlackRook => Black,
            BlackQueen => Black,
            BlackKing => Black,
        }
    }
}
