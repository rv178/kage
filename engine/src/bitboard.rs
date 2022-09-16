use crate::{GameStatus, Piece, Square};
use std::ops::{BitAnd, BitOr, Mul};

#[derive(Debug, Clone, Copy)]
pub struct BitBoard(pub u64);

#[derive(Debug, Clone)]
pub struct BitPos {
    pub wp: BitBoard, // white pawns
    pub wn: BitBoard, // white knights
    pub wb: BitBoard, // white bishops
    pub wr: BitBoard, // white rooks
    pub wq: BitBoard, // white queen
    pub wk: BitBoard, // white king
    pub bp: BitBoard, // black pawns
    pub bn: BitBoard, // black knights
    pub bb: BitBoard, // black bishops
    pub br: BitBoard, // black rooks
    pub bq: BitBoard, // black queen
    pub bk: BitBoard, // black king
}

impl BitPos {
    fn new(pieces: [Option<Piece>; 64]) -> BitPos {
        BitPos {
            wp: BitBoard::gen(&pieces, 'P'),
            wn: BitBoard::gen(&pieces, 'N'),
            wb: BitBoard::gen(&pieces, 'B'),
            wr: BitBoard::gen(&pieces, 'R'),
            wq: BitBoard::gen(&pieces, 'Q'),
            wk: BitBoard::gen(&pieces, 'K'),
            bp: BitBoard::gen(&pieces, 'p'),
            bn: BitBoard::gen(&pieces, 'n'),
            bb: BitBoard::gen(&pieces, 'b'),
            br: BitBoard::gen(&pieces, 'r'),
            bq: BitBoard::gen(&pieces, 'q'),
            bk: BitBoard::gen(&pieces, 'k'),
        }
    }
}

pub fn convert(game_status: GameStatus) {
    let positions: BitPos = BitPos::new(game_status.pieces);
    println!();
    println!("White pawns: ");
    positions.wp.print();
    // example
    let set: BitBoard = BitBoard(1 << Square::E2 as i32);
    set.print();
}

impl BitBoard {
    pub fn gen(pieces: &[Option<Piece>; 64], compare: char) -> Self {
        let mut bin_str = String::new();
        for piece in pieces {
            if let Some(piece) = piece {
                if !(piece.symbol == compare) {
                    bin_str.push('0');
                } else {
                    bin_str.push('1');
                }
            } else {
                bin_str.push('0');
            }
        }

        // convert binary string to decimal (u64)
        Self(u64::from_str_radix(&bin_str, 2).unwrap())
    }
    // check if bitboard is empty (u64 = 0)
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
    // check if equal
    pub fn eq(self, other: Self) -> bool {
        self.0 == other.0
    }
    // init empty bitboard
    pub fn empty() -> Self {
        Self(0)
    }
    // print bitboard
    pub fn print(&self) {
        println!("Value: {}", &self.0);
        for rank in 0..8 {
            print!("{}  ", rank + 1);
            for file in 0..8 {
                let square = rank * 8 + file;
                if self.0 & (1 << square) >= 1 {
                    print!("1 ");
                } else {
                    print!("0 ");
                }
            }
            println!();
        }
        println!();
        println!("   h g f e d c b a");
        println!();
    }
    pub fn set_bit(self, square: Square) -> Self {
        Self(self.0 & (1 << square as i32))
    }
}

impl BitAnd for BitBoard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitOr for BitBoard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl Mul for BitBoard {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0.wrapping_mul(rhs.0))
    }
}
