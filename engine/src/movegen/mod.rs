use crate::bitboard::*;
use crate::*;

pub mod king;
pub mod knight;
pub mod pawn;
pub mod sliding;

// hyperbola quintessence
pub fn hyperbola(sq: Square, occ: BitBoard, mask: u64) -> BitBoard {
    let mut forward = occ.0 & mask;
    let mut reverse = forward.reverse_bits();

    forward = forward.wrapping_sub(BitBoard::from_sq(sq).0);
    reverse = reverse.wrapping_sub(BitBoard::from_sq(sq).0.reverse_bits());
    forward ^= reverse.reverse_bits();
    forward &= mask;

    BitBoard(forward)

    //let r = BitBoard::from_sq(sq).0;
    //let r_dash = r.reverse_bits();
    //let o = occ.0 & mask;
    //let o_dash = o.reverse_bits();

    //let left = o ^ (o - 2 * r);
    //let right = o ^ (o_dash - 2 * r_dash).reverse_bits();
    // (((o&m) - 2*r) ^ ((o&m)`-2*r`)`) & m
    //let line_atk = ((o - 2 * r) ^ (o_dash - 2 * r_dash).reverse_bits()) & mask;
    //BitBoard(line_atk)
}

pub fn rook(sq: Square, occ: BitBoard) -> BitBoard {
    let tr = sq as usize / 8;
    let tf = sq as usize % 8;

    println!("tr: {}, tf: {}", tr, tf);

    BitBoard(hyperbola(sq, occ, FILES[tf].0).0 | hyperbola(sq, occ, RANKS[tr].0).0)
}

pub fn knight(square: Square) -> BitBoard {
    let board = BitBoard::from_sq(square);

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

pub fn king(square: Square) -> BitBoard {
    let board = BitBoard::from_sq(square);

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
