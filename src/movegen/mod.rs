pub mod pawns;

use crate::board::position::Position;
use crate::piece::{Piece, PieceType};

fn gen_moves_for_piece(position: &Position, piece: Piece) {
    use PieceType::*;

    match piece.piece_type {
        WhitePawn => pawns::white_pawn_moves(piece.square.bitboard(), position),
        _ => 0,
    };
}
