use crate::{GameStatus, Piece};

#[derive(Debug, Clone)]
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
    println!("{:?}", positions);
}

impl BitBoard {
    pub fn gen(pieces: &[Option<Piece>; 64], compare: char) -> BitBoard {
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

        BitBoard(u64::from_str_radix(&bin_str, 2).unwrap())
    }
}
