use crate::bitboard::{BitBoard, NOT_A_FILE, NOT_H_FILE};

pub fn bp_east_atk(board: BitBoard) -> BitBoard {
    BitBoard((board.0 << 9) & NOT_A_FILE.0)
}

pub fn bp_west_atk(board: BitBoard) -> BitBoard {
    BitBoard((board.0 << 7) & NOT_H_FILE.0)
}

pub fn bp_all_atk(board: BitBoard) -> BitBoard {
    let east = bp_east_atk(board);
    let west = bp_west_atk(board);
    BitBoard(east.0 | west.0)
}

/*
pub fn wp_east_atk(board: BitBoard) -> BitBoard {
    BitBoard((board.0 << 18) & NOT_A_FILE.0)
}

pub fn wp_west_atk(board: BitBoard) -> BitBoard {
    BitBoard((board.0 << 14) & NOT_H_FILE.0)
}

pub fn wp_all_atk(board: BitBoard) -> BitBoard {
    let east = wp_east_atk(board);
    let west = wp_west_atk(board);
    BitBoard(east.0 | west.0)
}
*/
