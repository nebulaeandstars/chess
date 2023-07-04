use super::bitboard::*;
use crate::piece::{PieceColor, PieceType};
use crate::types::*;

/// A Position (board state) represented using a series of BitBoards.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Position {
    // TODO: It might make more sense to eventually use arrays instead of
    // fields, as indexing wouldn't require a match statement.
    white:              SingleColorPosition,
    black:              SingleColorPosition,
    all_pieces:         BitBoard,
    en_passant_targets: BitBoard,
}

impl Position {
    /// Constructs a new Position, given SingleColorPositions for both
    /// the white pieces and black and the current en passant targets.
    pub const fn new(
        white: SingleColorPosition,
        black: SingleColorPosition,
        en_passant_targets: BitBoard,
    ) -> Self {
        let all_pieces = white.pieces | black.pieces;

        Self { white, black, all_pieces, en_passant_targets }
    }

    pub const fn starting_position() -> Self {
        PositionBuilder::new()
            .white_pawns(RANK_2)
            .white_knights(RANK_1 & (FILE_B | FILE_G))
            .white_bishops(RANK_1 & (FILE_C | FILE_F))
            .white_rooks(RANK_1 & (FILE_A | FILE_H))
            .white_queens(RANK_1 & FILE_D)
            .white_kings(RANK_1 & FILE_E)
            .black_pawns(RANK_7)
            .black_knights(RANK_8 & (FILE_B | FILE_G))
            .black_bishops(RANK_8 & (FILE_C | FILE_F))
            .black_rooks(RANK_8 & (FILE_A | FILE_H))
            .black_queens(RANK_8 & FILE_D)
            .black_kings(RANK_8 & FILE_E)
            .build()
    }

    pub fn en_passant_targets(&self) -> BitBoard { self.en_passant_targets }

    pub fn set_en_passant_targets(&mut self, targets: BitBoard) {
        self.en_passant_targets = targets;
    }

    pub const fn all_pieces(&self) -> BitBoard { self.all_pieces }
    pub const fn empty_squares(&self) -> BitBoard { !self.all_pieces }

    pub const fn white_pawns(&self) -> BitBoard { self.white.pawns }
    pub const fn white_knights(&self) -> BitBoard { self.white.knights }
    pub const fn white_bishops(&self) -> BitBoard { self.white.bishops }
    pub const fn white_rooks(&self) -> BitBoard { self.white.rooks }
    pub const fn white_queens(&self) -> BitBoard { self.white.queens }
    pub const fn white_kings(&self) -> BitBoard { self.white.kings }
    pub const fn white_pieces(&self) -> BitBoard { self.white.pieces }

    pub const fn black_pawns(&self) -> BitBoard { self.black.pawns }
    pub const fn black_knights(&self) -> BitBoard { self.black.knights }
    pub const fn black_bishops(&self) -> BitBoard { self.black.bishops }
    pub const fn black_rooks(&self) -> BitBoard { self.black.rooks }
    pub const fn black_queens(&self) -> BitBoard { self.black.queens }
    pub const fn black_kings(&self) -> BitBoard { self.black.kings }
    pub const fn black_pieces(&self) -> BitBoard { self.black.pieces }
}

impl std::ops::Index<PieceColor> for Position {
    type Output = BitBoard;
    fn index(&self, color: PieceColor) -> &Self::Output {
        match color {
            PieceColor::White => &self.white.pieces,
            PieceColor::Black => &self.black.pieces,
        }
    }
}

impl std::ops::Index<Square> for Position {
    type Output = Option<PieceType>;
    fn index(&self, _square: Square) -> &Self::Output {
        // TODO: implement this
        unimplemented!()
    }
}

impl std::ops::Index<PieceType> for Position {
    type Output = BitBoard;
    fn index(&self, piece: PieceType) -> &Self::Output {
        use PieceType::*;
        match piece {
            WhitePawn => &self.white.pawns,
            WhiteKnight => &self.white.knights,
            WhiteBishop => &self.white.bishops,
            WhiteRook => &self.white.rooks,
            WhiteQueen => &self.white.queens,
            WhiteKing => &self.white.kings,
            BlackPawn => &self.black.pawns,
            BlackKnight => &self.black.knights,
            BlackBishop => &self.black.bishops,
            BlackRook => &self.black.rooks,
            BlackQueen => &self.black.queens,
            BlackKing => &self.black.kings,
        }
    }
}

/// A set of BitBoards corresponding to each piece type, independent of color.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SingleColorPosition {
    // TODO: It might make more sense to eventually use arrays instead of
    // fields, as indexing wouldn't require a match statement.
    pawns:   BitBoard,
    knights: BitBoard,
    bishops: BitBoard,
    rooks:   BitBoard,
    queens:  BitBoard,
    kings:   BitBoard,
    pieces:  BitBoard,
}

impl SingleColorPosition {
    /// Constructs a new SingleColorPosition, given BitBoards for all
    /// similarly-colored piece types.
    pub const fn new(
        pawns: BitBoard,
        knights: BitBoard,
        bishops: BitBoard,
        rooks: BitBoard,
        queens: BitBoard,
        kings: BitBoard,
    ) -> Self {
        let pieces = pawns | knights | bishops | rooks | queens | kings;

        Self { pawns, knights, bishops, rooks, queens, kings, pieces }
    }

    pub const fn pawns(&self) -> &BitBoard { &self.pawns }
    pub const fn knights(&self) -> &BitBoard { &self.knights }
    pub const fn bishops(&self) -> &BitBoard { &self.bishops }
    pub const fn rooks(&self) -> &BitBoard { &self.rooks }
    pub const fn queens(&self) -> &BitBoard { &self.queens }
    pub const fn kings(&self) -> &BitBoard { &self.kings }
}


/// Builder for Positions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PositionBuilder {
    pub white_pawns:   BitBoard,
    pub white_knights: BitBoard,
    pub white_bishops: BitBoard,
    pub white_rooks:   BitBoard,
    pub white_queens:  BitBoard,
    pub white_kings:   BitBoard,

    pub black_pawns:   BitBoard,
    pub black_knights: BitBoard,
    pub black_bishops: BitBoard,
    pub black_rooks:   BitBoard,
    pub black_queens:  BitBoard,
    pub black_kings:   BitBoard,

    pub en_passant_targets: BitBoard,
}

impl PositionBuilder {
    pub const fn new() -> Self {
        Self {
            white_pawns:        0,
            white_knights:      0,
            white_bishops:      0,
            white_rooks:        0,
            white_queens:       0,
            white_kings:        0,
            black_pawns:        0,
            black_knights:      0,
            black_bishops:      0,
            black_rooks:        0,
            black_queens:       0,
            black_kings:        0,
            en_passant_targets: 0,
        }
    }

    pub const fn build(&self) -> Position {
        Position::new(
            self.get_white_pieces(),
            self.get_black_pieces(),
            self.en_passant_targets,
        )
    }

    pub const fn get_white_pieces(&self) -> SingleColorPosition {
        SingleColorPosition::new(
            self.white_pawns,
            self.white_knights,
            self.white_bishops,
            self.white_rooks,
            self.white_queens,
            self.white_kings,
        )
    }

    pub const fn get_black_pieces(&self) -> SingleColorPosition {
        SingleColorPosition::new(
            self.black_pawns,
            self.black_knights,
            self.black_bishops,
            self.black_rooks,
            self.black_queens,
            self.black_kings,
        )
    }

    pub const fn white_pawns(mut self, white_pawns: BitBoard) -> Self {
        self.white_pawns = white_pawns;
        self
    }
    pub const fn white_knights(mut self, white_knights: BitBoard) -> Self {
        self.white_knights = white_knights;
        self
    }
    pub const fn white_bishops(mut self, white_bishops: BitBoard) -> Self {
        self.white_bishops = white_bishops;
        self
    }
    pub const fn white_rooks(mut self, white_rooks: BitBoard) -> Self {
        self.white_rooks = white_rooks;
        self
    }
    pub const fn white_queens(mut self, white_queens: BitBoard) -> Self {
        self.white_queens = white_queens;
        self
    }
    pub const fn white_kings(mut self, white_kings: BitBoard) -> Self {
        self.white_kings = white_kings;
        self
    }

    pub const fn black_pawns(mut self, black_pawns: BitBoard) -> Self {
        self.black_pawns = black_pawns;
        self
    }
    pub const fn black_knights(mut self, black_knights: BitBoard) -> Self {
        self.black_knights = black_knights;
        self
    }
    pub const fn black_bishops(mut self, black_bishops: BitBoard) -> Self {
        self.black_bishops = black_bishops;
        self
    }
    pub const fn black_rooks(mut self, black_rooks: BitBoard) -> Self {
        self.black_rooks = black_rooks;
        self
    }
    pub const fn black_queens(mut self, black_queens: BitBoard) -> Self {
        self.black_queens = black_queens;
        self
    }
    pub const fn black_kings(mut self, black_kings: BitBoard) -> Self {
        self.black_kings = black_kings;
        self
    }

    pub const fn en_passant_targets(mut self, targets: BitBoard) -> Self {
        self.en_passant_targets = targets;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starting_position_is_correct() {
        let board1 = Position::starting_position();
        let board2 = PositionBuilder::new()
            .white_pawns(RANK_2)
            .white_knights(RANK_1 & (FILE_B | FILE_G))
            .white_bishops(RANK_1 & (FILE_C | FILE_F))
            .white_rooks(RANK_1 & (FILE_A | FILE_H))
            .white_queens(RANK_1 & FILE_D)
            .white_kings(RANK_1 & FILE_E)
            .black_pawns(RANK_7)
            .black_knights(RANK_8 & (FILE_B | FILE_G))
            .black_bishops(RANK_8 & (FILE_C | FILE_F))
            .black_rooks(RANK_8 & (FILE_A | FILE_H))
            .black_queens(RANK_8 & FILE_D)
            .black_kings(RANK_8 & FILE_E)
            .build();

        assert_eq!(board1, board2);
    }
}
