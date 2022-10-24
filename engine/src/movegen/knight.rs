use crate::utils::match_u32_to_sq;
use crate::{bitboard::*, Square};

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

pub fn lookup(sq: Square) -> BitBoard {
    let board = BitBoard::from_sq(sq);

    let no_no_east = no_no_east(board);
    let no_no_west = no_no_west(board);
    let so_so_east = so_so_east(board);
    let so_so_west = so_so_west(board);
    let no_ea_east = no_ea_east(board);
    let so_ea_east = so_ea_east(board);
    let no_we_west = no_we_west(board);
    let so_we_west = so_we_west(board);

    BitBoard(
        no_no_east.0
            | no_no_west.0
            | so_so_east.0
            | so_so_west.0
            | no_ea_east.0
            | so_ea_east.0
            | no_we_west.0
            | so_we_west.0,
    )
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
