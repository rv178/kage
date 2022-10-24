use crate::utils::match_u32_to_sq;
use crate::{bitboard::*, Square};

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

pub fn lookup(sq: Square) -> BitBoard {
    let board = BitBoard::from_sq(sq);

    let no = no(board);
    let so = so(board);
    let ea = ea(board);
    let we = we(board);
    let no_ea = no_ea(board);
    let no_we = no_we(board);
    let so_ea = so_ea(board);
    let so_we = so_we(board);

    BitBoard(no.0 | so.0 | ea.0 | we.0 | no_ea.0 | no_we.0 | so_ea.0 | so_we.0)
}

pub fn all(board: BitBoard, occ: BitBoard) -> BitBoard {
    let mut bb = BitBoard::empty();

    for rank in 0..8 {
        for file in 0..8 {
            let square = rank * 8 + file;

            if board.0 & (1 << square) != 0 {
                let board = lookup(match_u32_to_sq(square));
                bb.0 |= board.0;
            }
        }
    }
    BitBoard(bb.0 - (bb.0 & occ.0))
}
