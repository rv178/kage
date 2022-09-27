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

pub fn north_east_knight_atk(board: BitBoard) -> BitBoard {
    BitBoard((board.0 << 17) & NOT_A_FILE.0)
}

pub fn north_west_knight_atk(board: BitBoard) -> BitBoard {
    BitBoard((board.0 << 15) & NOT_H_FILE.0)
}

pub fn south_east_knight_atk(board: BitBoard) -> BitBoard {
    BitBoard((board.0 >> 15) & NOT_A_FILE.0)
}

pub fn south_west_knight_atk(board: BitBoard) -> BitBoard {
    BitBoard((board.0 >> 17) & NOT_H_FILE.0)
}

pub fn east_north_knight_atk(board: BitBoard) -> BitBoard {
    BitBoard((board.0 << 10) & NOT_AB_FILE.0)
}

pub fn east_south_knight_atk(board: BitBoard) -> BitBoard {
    BitBoard((board.0 >> 6) & NOT_AB_FILE.0)
}

pub fn west_north_knight_atk(board: BitBoard) -> BitBoard {
    BitBoard((board.0 << 6) & NOT_HG_FILE.0)
}

pub fn west_south_knight_atk(board: BitBoard) -> BitBoard {
    BitBoard((board.0 >> 10) & NOT_HG_FILE.0)
}

pub fn knight_atk_lookup(square: Square) -> BitBoard {
    let board = BitBoard::from_sq(square);

    let north_east = north_east_knight_atk(board);
    let north_west = north_west_knight_atk(board);
    let south_east = south_east_knight_atk(board);
    let south_west = south_west_knight_atk(board);
    let east_north = east_north_knight_atk(board);
    let east_south = east_south_knight_atk(board);
    let west_north = west_north_knight_atk(board);
    let west_south = west_south_knight_atk(board);

    BitBoard(
        north_east.0
            | north_west.0
            | south_east.0
            | south_west.0
            | east_north.0
            | east_south.0
            | west_north.0
            | west_south.0,
    )
}

// king attacks

pub fn north_king_atk(board: BitBoard) -> BitBoard {
    BitBoard(board.0 << 8)
}

pub fn south_king_atk(board: BitBoard) -> BitBoard {
    BitBoard(board.0 >> 8)
}

pub fn east_king_atk(board: BitBoard) -> BitBoard {
    BitBoard((board.0 << 1) & NOT_A_FILE.0)
}

pub fn west_king_atk(board: BitBoard) -> BitBoard {
    BitBoard((board.0 >> 1) & NOT_H_FILE.0)
}

pub fn north_east_king_atk(board: BitBoard) -> BitBoard {
    BitBoard((board.0 << 9) & NOT_A_FILE.0)
}

pub fn north_west_king_atk(board: BitBoard) -> BitBoard {
    BitBoard((board.0 << 7) & NOT_H_FILE.0)
}

pub fn south_east_king_atk(board: BitBoard) -> BitBoard {
    BitBoard((board.0 >> 7) & NOT_A_FILE.0)
}

pub fn south_west_king_atk(board: BitBoard) -> BitBoard {
    BitBoard((board.0 >> 9) & NOT_H_FILE.0)
}

pub fn king_atk_lookup(square: Square) -> BitBoard {
    let board = BitBoard::from_sq(square);

    let north = north_king_atk(board);
    let south = south_king_atk(board);
    let east = east_king_atk(board);
    let west = west_king_atk(board);
    let north_east = north_east_king_atk(board);
    let north_west = north_west_king_atk(board);
    let south_east = south_east_king_atk(board);
    let south_west = south_west_king_atk(board);

    BitBoard(
        north.0
            | south.0
            | east.0
            | west.0
            | north_east.0
            | north_west.0
            | south_east.0
            | south_west.0,
    )
}
