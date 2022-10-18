use crate::bitboard::*;
use crate::*;

// generate bishop attack mask
pub fn bishop_atk_mask(square: Square) -> BitBoard {
    let mut atk = BitBoard::empty();

    let mut rank;
    let mut file;

    let tr = square as i8 / 8;
    let tf = square as i8 % 8;

    // (1) south east
    for i in 1..7 {
        rank = tr + i;
        file = tf + i;

        if rank <= 6 && file <= 6 {
            atk.0 |= 1 << (rank * 8 + file);
        } else {
            break;
        }
    }

    // (2) north east
    for i in 1..7 {
        rank = tr - i;
        file = tf + i;

        if rank >= 1 && file <= 6 {
            atk.0 |= 1 << (rank * 8 + file);
        } else {
            break;
        }
    }

    // (3) south west
    for i in 1..7 {
        rank = tr + i;
        file = tf - i;

        if rank <= 6 && file >= 1 {
            atk.0 |= 1 << (rank * 8 + file);
        } else {
            break;
        }
    }

    // (4) north west
    for i in 1..7 {
        rank = tr - i;
        file = tf - i;

        if rank >= 1 && file >= 1 {
            atk.0 |= 1 << (rank * 8 + file);
        } else {
            break;
        }
    }

    atk
}

// generate rook attack mask

pub fn rook_atk_mask(square: Square) -> BitBoard {
    let mut atk = BitBoard::empty();

    let mut rank;

    let tr = square as i8 / 8;
    let tf = square as i8 % 8;

    // (1) north
    for i in 1..7 {
        rank = tr + i;

        if rank <= 6 {
            atk.0 |= 1 << (rank * 8 + tf);
        } else {
            break;
        }
    }

    // (2) south
    for i in 1..7 {
        rank = tr - i;

        if rank >= 1 {
            atk.0 |= 1 << (rank * 8 + tf);
        } else {
            break;
        }
    }

    // (3) east
    for i in 1..7 {
        let file = tf + i;

        if file <= 6 {
            atk.0 |= 1 << (tr * 8 + file);
        } else {
            break;
        }
    }

    // (4) west
    for i in 1..7 {
        let file = tf - i;

        if file >= 1 {
            atk.0 |= 1 << (tr * 8 + file);
        } else {
            break;
        }
    }

    atk
}
