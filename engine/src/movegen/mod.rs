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

// lookup rook attacks for a rook on a particular square
pub fn rook(sq: Square, occ: BitBoard) -> BitBoard {
    let tr = sq as usize / 8;
    let tf = sq as usize % 8;

    BitBoard(hyp_quint(sq, occ, FILES[tf].0).0 | hyp_quint(sq, occ, RANKS[tr].0).0)
}

// lookup rook attacks for all bits (bitboard containing rook occupancies)
pub fn rook_bb(board: BitBoard, occ: BitBoard) -> BitBoard {
    let mut bb = BitBoard::empty();

    for rank in 0..8 {
        for file in 0..8 {
            let square = rank * 8 + file;

            let tr = square as usize / 8;
            let tf = square as usize % 8;

            if board.0 & (1 << square) != 0 {
                bb.0 |= hyp_quint(match_u32_to_sq(square), occ, FILES[tf].0).0;
                bb.0 |= hyp_quint(match_u32_to_sq(square), occ, RANKS[tr].0).0;
            }
        }
    }
    bb
}

// lookup bishop attacks for a bishop on a particular square
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

// lookup bishop attacks for all bits (bitboard containing bishop occupancies)
pub fn bishop_bb(board: BitBoard, occ: BitBoard) -> BitBoard {
    let mut bb = BitBoard::empty();

    for rank in 0..8 {
        for file in 0..8 {
            let square = rank * 8 + file;

            let tr = square as usize / 8;
            let tf = square as usize % 8;

            let diag_index: usize = 7 + tr - tf;
            let anti_diag_index: usize = tr + tf;

            if board.0 & (1 << square) != 0 {
                bb.0 |= hyp_quint(match_u32_to_sq(square), occ, DIAG[diag_index].0).0;
                bb.0 |= hyp_quint(match_u32_to_sq(square), occ, ANTI_DIAG[anti_diag_index].0).0;
            }
        }
    }
    bb
}

// lookup queen attacks for a queen on a particular square
pub fn queen(sq: Square, occ: BitBoard) -> BitBoard {
    BitBoard(rook(sq, occ).0 | bishop(sq, occ).0)
}

// lookup queen attacks for all bits (bitboard containing queen occupancies)
pub fn queen_bb(board: BitBoard, occ: BitBoard) -> BitBoard {
    let mut bb = BitBoard::empty();

    for rank in 0..8 {
        for file in 0..8 {
            let square = rank * 8 + file;

            if board.0 & (1 << square) != 0 {
                bb.0 |= rook_bb(BitBoard::from_sq(match_u32_to_sq(square)), occ).0;
                bb.0 |= bishop_bb(BitBoard::from_sq(match_u32_to_sq(square)), occ).0;
            }
        }
    }
    bb
}
