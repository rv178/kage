use crate::bitboard::*;
use crate::*;

// hyperbola quintessence
pub fn hyperbola(sq: Square, occ: BitBoard, mask: u64) -> BitBoard {
    let mut forward = occ.0 & mask;
    let mut reverse = forward.reverse_bits();

    forward -= BitBoard::from_sq(sq).0;
    reverse -= BitBoard::from_sq(sq).0.reverse_bits();
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

pub fn rook_atk(sq: Square, occ: BitBoard) -> BitBoard {
    let tr = sq as usize / 8;
    let tf = sq as usize % 8;

    println!("tr: {}, tf: {}", tr, tf);

    BitBoard(hyperbola(sq, occ, FILES[tf].0).0 | hyperbola(sq, occ, RANKS[tr].0).0)
}
