use crate::board::bitboard::*;
use crate::board::position::Position;

/// Finds all potential moves for white pawns represented by the given
/// BitBoard, within the context of the current position. Does not account for
/// en-passant.
pub fn white_pawn_moves(
    white_pawns: BitBoard,
    position: &Position,
) -> BitBoard {
    let pushes = white_pawn_pushes(white_pawns, position.empty_squares());
    let captures = white_pawn_captures(
        white_pawns,
        position.black_pieces() | position.en_passant_targets(),
    );
    pushes | captures
}

/// Finds all potential moves for black pawns represented by the given
/// BitBoard, within the context of the current position. Does not account for
/// en-passant.
pub fn black_pawn_moves(
    black_pawns: BitBoard,
    position: &Position,
) -> BitBoard {
    let pushes = black_pawn_pushes(black_pawns, position.empty_squares());
    let captures = black_pawn_captures(
        black_pawns,
        position.white_pieces() | position.en_passant_targets(),
    );
    pushes | captures
}

fn white_pawn_pushes(
    white_pawns: BitBoard,
    empty_squares: BitBoard,
) -> BitBoard {
    let one_square = white_pawns.inc_rank() & empty_squares;
    let two_squares = one_square.inc_rank() & RANK_4 & empty_squares;
    one_square | two_squares
}

fn black_pawn_pushes(
    black_pawns: BitBoard,
    empty_squares: BitBoard,
) -> BitBoard {
    let one_square = black_pawns.dec_rank() & empty_squares;
    let two_squares = one_square.dec_rank() & RANK_5 & empty_squares;
    one_square | two_squares
}

fn white_pawn_attacks(white_pawns: BitBoard) -> BitBoard {
    let diag_right = (white_pawns << 9) & !FILE_A;
    let diag_left = (white_pawns << 7) & !FILE_H;
    diag_right | diag_left
}

fn black_pawn_attacks(black_pawns: BitBoard) -> BitBoard {
    let diag_left = (black_pawns >> 9) & !FILE_H;
    let diag_right = (black_pawns >> 7) & !FILE_A;
    diag_right | diag_left
}

fn white_pawn_captures(
    white_pawns: BitBoard,
    black_pieces: BitBoard,
) -> BitBoard {
    white_pawn_attacks(white_pawns) & black_pieces
}

fn black_pawn_captures(
    black_pawns: BitBoard,
    white_pieces: BitBoard,
) -> BitBoard {
    black_pawn_attacks(black_pawns) & white_pieces
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::position::PositionBuilder;

    const DUMMY_WHITE_PAWNS: BitBoard = 0x00_00_00_08_20_00_11_00;
    const DUMMY_BLACK_PAWNS: BitBoard = 0x00_0c_00_20_00_82_00_00;

    const DUMMY_POSITION: Position = PositionBuilder::new()
        .white_pawns(DUMMY_WHITE_PAWNS)
        .black_pawns(DUMMY_BLACK_PAWNS)
        .build();

    #[test]
    fn white_pawns_can_push() {
        let white_pawn_pushes = white_pawn_pushes(
            DUMMY_POSITION.white_pawns(),
            DUMMY_POSITION.empty_squares(),
        );
        assert_eq!(white_pawn_pushes, 0x80011110000);
    }

    #[test]
    fn white_pawns_can_capture() {
        let white_pawn_captures = white_pawn_captures(
            DUMMY_POSITION.white_pawns(),
            DUMMY_POSITION.black_pieces(),
        );
        assert_eq!(white_pawn_captures, 0x20000);
    }

    #[test]
    fn white_pawns_can_move() {
        let white_pawn_moves =
            white_pawn_moves(DUMMY_POSITION.white_pawns(), &DUMMY_POSITION);
        assert_eq!(white_pawn_moves, 0x80011110000 | 0x20000);
    }

    #[test]
    fn black_pawns_can_push() {
        let black_pawn_pushes = black_pawn_pushes(
            DUMMY_POSITION.black_pawns(),
            DUMMY_POSITION.empty_squares(),
        );
        assert_eq!(black_pawn_pushes, 0xc0400008200);
    }

    #[test]
    fn black_pawns_can_capture() {
        let black_pawn_captures = black_pawn_captures(
            DUMMY_POSITION.black_pawns(),
            DUMMY_POSITION.white_pieces(),
        );
        assert_eq!(black_pawn_captures, 0x100);
    }

    #[test]
    fn black_pawns_can_move() {
        let black_pawn_moves =
            black_pawn_moves(DUMMY_POSITION.black_pawns(), &DUMMY_POSITION);
        assert_eq!(black_pawn_moves, 0xc0400008200 | 0x100);
    }

    #[test]
    fn en_passant_is_allowed() {
        let position = PositionBuilder::new()
            .white_pawns(0x800000000)
            .black_pawns(0x400000000)
            .en_passant_targets(0x40008000000)
            .build();

        let white_pawn_moves =
            white_pawn_moves(position.white_pawns(), &position);
        let black_pawn_moves =
            black_pawn_moves(position.black_pawns(), &position);

        assert_eq!(white_pawn_moves, 0xc0000000000);
        assert_eq!(black_pawn_moves, 0xc000000);
    }
}
