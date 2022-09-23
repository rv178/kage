use crate::bitboard::{BitBoard, NOT_A_FILE, NOT_H_FILE};

pub fn b_pawn_east_attacks(board: BitBoard) -> BitBoard {
    BitBoard((board.0 << 9) & NOT_A_FILE.0)
}

pub fn b_pawn_west_attacks(board: BitBoard) -> BitBoard {
    BitBoard((board.0 << 7) & NOT_H_FILE.0)
}

pub fn b_pawn_any_attacks(east: BitBoard, west: BitBoard) -> BitBoard {
    BitBoard(east.0 | west.0)
}
