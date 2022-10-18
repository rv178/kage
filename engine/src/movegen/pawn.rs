use crate::bitboard::*;
use crate::*;
use std::process::exit;

pub fn east(board: BitBoard, colour: Colour) -> BitBoard {
    match colour {
        Colour::White => BitBoard((board.0 >> 9) & NOT_H_FILE.0),
        Colour::Black => BitBoard((board.0 << 9) & NOT_A_FILE.0),
        Colour::Undefined => exit(1),
    }
}

pub fn west(board: BitBoard, colour: Colour) -> BitBoard {
    match colour {
        Colour::White => BitBoard((board.0 >> 7) & NOT_A_FILE.0),
        Colour::Black => BitBoard((board.0 << 7) & NOT_H_FILE.0),
        Colour::Undefined => exit(1),
    }
}

pub fn lookup(square: Square, colour: Colour) -> BitBoard {
    let board = BitBoard::from_sq(square);

    match colour {
        Colour::White => BitBoard(east(board, Colour::White).0 | west(board, Colour::White).0),
        Colour::Black => BitBoard(east(board, Colour::Black).0 | west(board, Colour::Black).0),
        Colour::Undefined => exit(1),
    }
}

pub fn all(board: BitBoard, colour: Colour) -> BitBoard {
    match colour {
        Colour::White => BitBoard(east(board, Colour::White).0 | west(board, Colour::White).0),
        Colour::Black => BitBoard(east(board, Colour::Black).0 | west(board, Colour::Black).0),
        Colour::Undefined => exit(1),
    }
}
