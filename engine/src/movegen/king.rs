use crate::bitboard::*;

// north
pub fn no(board: BitBoard) -> BitBoard {
    BitBoard(board.0 << 8)
}

// south
pub fn so(board: BitBoard) -> BitBoard {
    BitBoard(board.0 >> 8)
}

// east
pub fn ea(board: BitBoard) -> BitBoard {
    BitBoard((board.0 << 1) & NOT_A_FILE.0)
}

// west
pub fn we(board: BitBoard) -> BitBoard {
    BitBoard((board.0 >> 1) & NOT_H_FILE.0)
}

// north east
pub fn no_ea(board: BitBoard) -> BitBoard {
    BitBoard((board.0 << 9) & NOT_A_FILE.0)
}

// north west
pub fn no_we(board: BitBoard) -> BitBoard {
    BitBoard((board.0 << 7) & NOT_H_FILE.0)
}

// south east
pub fn so_ea(board: BitBoard) -> BitBoard {
    BitBoard((board.0 >> 7) & NOT_A_FILE.0)
}

// south west
pub fn so_we(board: BitBoard) -> BitBoard {
    BitBoard((board.0 >> 9) & NOT_H_FILE.0)
}
