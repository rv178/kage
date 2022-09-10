use crate::{GameStatus, Piece};

#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
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
            wp: BitBoard(gen_bitboard(&pieces, 'P')),
            wn: BitBoard(gen_bitboard(&pieces, 'N')),
            wb: BitBoard(gen_bitboard(&pieces, 'B')),
            wr: BitBoard(gen_bitboard(&pieces, 'R')),
            wq: BitBoard(gen_bitboard(&pieces, 'Q')),
            wk: BitBoard(gen_bitboard(&pieces, 'K')),
            bp: BitBoard(gen_bitboard(&pieces, 'p')),
            bn: BitBoard(gen_bitboard(&pieces, 'n')),
            bb: BitBoard(gen_bitboard(&pieces, 'b')),
            br: BitBoard(gen_bitboard(&pieces, 'r')),
            bq: BitBoard(gen_bitboard(&pieces, 'q')),
            bk: BitBoard(gen_bitboard(&pieces, 'k')),
        }
    }
}

pub fn convert(game_status: GameStatus) {
    let positions: BitPos = BitPos::new(game_status.pieces);
    println!("{:?}", positions);
}

pub fn gen_bitboard(pieces: &[Option<Piece>; 64], compare: char) -> u64 {
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

    u64::from_str_radix(&bin_str, 2).unwrap()
}
