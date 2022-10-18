use crate::bitboard::*;

pub fn no_no_east(board: BitBoard) -> BitBoard {
    BitBoard((board.0 << 17) & NOT_A_FILE.0)
}

pub fn no_no_west(board: BitBoard) -> BitBoard {
    BitBoard((board.0 << 15) & NOT_H_FILE.0)
}

pub fn so_so_east(board: BitBoard) -> BitBoard {
    BitBoard((board.0 >> 15) & NOT_A_FILE.0)
}

pub fn so_so_west(board: BitBoard) -> BitBoard {
    BitBoard((board.0 >> 17) & NOT_H_FILE.0)
}

pub fn no_ea_east(board: BitBoard) -> BitBoard {
    BitBoard((board.0 << 10) & NOT_AB_FILE.0)
}

pub fn so_ea_east(board: BitBoard) -> BitBoard {
    BitBoard((board.0 >> 6) & NOT_AB_FILE.0)
}

pub fn no_we_west(board: BitBoard) -> BitBoard {
    BitBoard((board.0 << 6) & NOT_HG_FILE.0)
}

pub fn so_we_west(board: BitBoard) -> BitBoard {
    BitBoard((board.0 >> 10) & NOT_HG_FILE.0)
}
