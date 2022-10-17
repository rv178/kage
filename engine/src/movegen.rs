use std::process::exit;

use crate::bitboard::{BitBoard, NOT_AB_FILE, NOT_A_FILE, NOT_HG_FILE, NOT_H_FILE};
use crate::{Colour, Square};

// pawn attacks

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

pub fn wp_east_atk(board: BitBoard) -> BitBoard {
    BitBoard((board.0 >> 9) & NOT_H_FILE.0)
}

pub fn wp_west_atk(board: BitBoard) -> BitBoard {
    BitBoard((board.0 >> 7) & NOT_A_FILE.0)
}

pub fn wp_all_atk(board: BitBoard) -> BitBoard {
    let east = wp_east_atk(board);
    let west = wp_west_atk(board);
    BitBoard(east.0 | west.0)
}

pub fn pawn_atk_lookup(square: Square, colour: Colour) -> BitBoard {
    let board = BitBoard::from_sq(square);

    match colour {
        Colour::White => wp_all_atk(board),
        Colour::Black => bp_all_atk(board),
        Colour::Undefined => exit(1),
    }
}

// knight attacks

pub fn no_no_east_knight(board: BitBoard) -> BitBoard {
    BitBoard((board.0 << 17) & NOT_A_FILE.0)
}

pub fn no_no_west_knight(board: BitBoard) -> BitBoard {
    BitBoard((board.0 << 15) & NOT_H_FILE.0)
}

pub fn so_so_east_knight(board: BitBoard) -> BitBoard {
    BitBoard((board.0 >> 15) & NOT_A_FILE.0)
}

pub fn so_so_west_knight(board: BitBoard) -> BitBoard {
    BitBoard((board.0 >> 17) & NOT_H_FILE.0)
}

pub fn no_ea_east_knight(board: BitBoard) -> BitBoard {
    BitBoard((board.0 << 10) & NOT_AB_FILE.0)
}

pub fn so_ea_east_knight(board: BitBoard) -> BitBoard {
    BitBoard((board.0 >> 6) & NOT_AB_FILE.0)
}

pub fn no_we_west_knight(board: BitBoard) -> BitBoard {
    BitBoard((board.0 << 6) & NOT_HG_FILE.0)
}

pub fn so_we_west_knight(board: BitBoard) -> BitBoard {
    BitBoard((board.0 >> 10) & NOT_HG_FILE.0)
}

pub fn knight_atk_lookup(square: Square) -> BitBoard {
    let board = BitBoard::from_sq(square);

    let no_no_east = no_no_east_knight(board);
    let no_no_west = no_no_west_knight(board);
    let so_so_east = so_so_east_knight(board);
    let so_so_west = so_so_west_knight(board);
    let no_ea_east = no_ea_east_knight(board);
    let so_ea_east = so_ea_east_knight(board);
    let no_we_west = no_we_west_knight(board);
    let so_we_west = so_we_west_knight(board);

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

// king attacks

pub fn no_king(board: BitBoard) -> BitBoard {
    BitBoard(board.0 << 8)
}

pub fn so_king(board: BitBoard) -> BitBoard {
    BitBoard(board.0 >> 8)
}

pub fn ea_king(board: BitBoard) -> BitBoard {
    BitBoard((board.0 << 1) & NOT_A_FILE.0)
}

pub fn we_king(board: BitBoard) -> BitBoard {
    BitBoard((board.0 >> 1) & NOT_H_FILE.0)
}

pub fn no_ea_king(board: BitBoard) -> BitBoard {
    BitBoard((board.0 << 9) & NOT_A_FILE.0)
}

pub fn no_we_king(board: BitBoard) -> BitBoard {
    BitBoard((board.0 << 7) & NOT_H_FILE.0)
}

pub fn so_ea_king(board: BitBoard) -> BitBoard {
    BitBoard((board.0 >> 7) & NOT_A_FILE.0)
}

pub fn so_we_king(board: BitBoard) -> BitBoard {
    BitBoard((board.0 >> 9) & NOT_H_FILE.0)
}

pub fn king_atk_lookup(square: Square) -> BitBoard {
    let board = BitBoard::from_sq(square);

    let no = no_king(board);
    let so = so_king(board);
    let ea = ea_king(board);
    let we = we_king(board);
    let no_ea = no_ea_king(board);
    let no_we = no_we_king(board);
    let so_ea = so_ea_king(board);
    let so_we = so_we_king(board);

    BitBoard(no.0 | so.0 | ea.0 | we.0 | no_ea.0 | no_we.0 | so_ea.0 | so_we.0)
}

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

// lookup bishop attacks

//pub fn bishop_atk_lookup(square: Square, block: BitBoard) -> BitBoard {
//let mut atk = BitBoard::empty();

//let mut rank;
//let mut file;

//let tr = square as i8 / 8;
//let tf = square as i8 % 8;

//// (1) south east
//for i in 1..7 {
//rank = tr + i;
//file = tf + i;

//if rank <= 7 && file <= 7 {
//atk.0 |= 1 << (rank * 8 + file);
//} else {
//break;
//}

//if 1 << (rank * 8 + file) & block.0 != 0 {
//break;
//};
//}

//// (2) north east
//for i in 1..7 {
//rank = tr - i;
//file = tf + i;

//if rank >= 0 && file <= 7 {
//atk.0 |= 1 << (rank * 8 + file);
//} else {
//break;
//}

//if 1 << (rank * 8 + file) & block.0 != 0 {
//break;
//};
//}

//// (3) south west
//for i in 1..7 {
//rank = tr + i;
//file = tf - i;

//if rank <= 7 && file >= 0 {
//atk.0 |= 1 << (rank * 8 + file);
//} else {
//break;
//}

//if 1 << (rank * 8 + file) & block.0 != 0 {
//break;
//};
//}

//// (4) north west
//for i in 1..7 {
//rank = tr - i;
//file = tf - i;

//if rank >= 0 && file >= 0 {
//atk.0 |= 1 << (rank * 8 + file);
//} else {
//break;
//}

//if 1 << (rank * 8 + file) & block.0 != 0 {
//break;
//};
//}

//atk
//}

//// lookup rook attacks

//pub fn rook_atk_lookup(square: Square, block: BitBoard) -> BitBoard {
//let mut atk = BitBoard::empty();

//let mut rank;

//let tr = square as i8 / 8;
//let tf = square as i8 % 8;

//// (1) north
//for i in 1..7 {
//rank = tr + i;

//if rank <= 7 {
//atk.0 |= 1 << (rank * 8 + tf);
//} else {
//break;
//}

//if 1 << (rank * 8 + tf) & block.0 != 0 {
//break;
//};
//}

//// (2) south
//for i in 1..7 {
//rank = tr - i;

//if rank >= 0 {
//atk.0 |= 1 << (rank * 8 + tf);
//} else {
//break;
//}

//if 1 << (rank * 8 + tf) & block.0 != 0 {
//break;
//};
//}

//// (3) east
//for i in 1..7 {
//let file = tf + i;

//if file <= 7 {
//atk.0 |= 1 << (tr * 8 + file);
//} else {
//break;
//}

//if 1 << (tr * 8 + file) & block.0 != 0 {
//break;
//};
//}

//// (4) west
//for i in 1..7 {
//let file = tf - i;

//if file >= 0 {
//atk.0 |= 1 << (tr * 8 + file);
//} else {
//break;
//}

//if 1 << (tr * 8 + file) & block.0 != 0 {
//break;
//};
//}

//atk
//}
