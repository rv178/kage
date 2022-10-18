use crate::bitboard::*;
use crate::*;

pub mod king;
pub mod knight;
pub mod pawn;

// hyperbola quintessence
pub fn hyp_quint(sq: Square, occ: BitBoard, mask: u64) -> BitBoard {
    let mut forward = occ.0 & mask;
    let mut reverse = forward.reverse_bits();

    forward = forward.wrapping_sub(BitBoard::from_sq(sq).0);
    reverse = reverse.wrapping_sub(BitBoard::from_sq(sq).0.reverse_bits());
    forward ^= reverse.reverse_bits();
    forward &= mask;

    BitBoard(forward)
}

pub fn rook(sq: Square, occ: BitBoard) -> BitBoard {
    let tr = sq as usize / 8;
    let tf = sq as usize % 8;

    BitBoard(hyp_quint(sq, occ, FILES[tf].0).0 | hyp_quint(sq, occ, RANKS[tr].0).0)
}

pub fn bishop(sq: Square, occ: BitBoard) -> BitBoard {
    let tr = sq as usize / 8;
    let tf = sq as usize % 8;

    let diag_index: usize = 7 + tr - tf;
    let anti_diag_index: usize = tr + tf;

    BitBoard(
        hyp_quint(sq, occ, DIAG[diag_index].0).0
            | hyp_quint(sq, occ, ANTI_DIAG[anti_diag_index].0).0,
    )
}

pub fn queen(sq: Square, occ: BitBoard) -> BitBoard {
    BitBoard(rook(sq, occ).0 | bishop(sq, occ).0)
}

pub fn knight(sq: Square) -> BitBoard {
    let board = BitBoard::from_sq(sq);

    let no_no_east = knight::no_no_east(board);
    let no_no_west = knight::no_no_west(board);
    let so_so_east = knight::so_so_east(board);
    let so_so_west = knight::so_so_west(board);
    let no_ea_east = knight::no_ea_east(board);
    let so_ea_east = knight::so_ea_east(board);
    let no_we_west = knight::no_we_west(board);
    let so_we_west = knight::so_we_west(board);

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

pub fn king(sq: Square) -> BitBoard {
    let board = BitBoard::from_sq(sq);

    let no = king::no(board);
    let so = king::so(board);
    let ea = king::ea(board);
    let we = king::we(board);
    let no_ea = king::no_ea(board);
    let no_we = king::no_we(board);
    let so_ea = king::so_ea(board);
    let so_we = king::so_we(board);

    BitBoard(no.0 | so.0 | ea.0 | we.0 | no_ea.0 | no_we.0 | so_ea.0 | so_we.0)
}
