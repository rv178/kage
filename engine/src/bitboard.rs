use crate::movegen::*;
use crate::utils::match_u32_to_sq;
use crate::{GameStatus, Piece, Square};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    fn new(pieces: &mut [Option<Piece>; 64]) -> BitPos {
        BitPos {
            wp: BitBoard::gen(pieces, 'P'),
            wn: BitBoard::gen(pieces, 'N'),
            wb: BitBoard::gen(pieces, 'B'),
            wr: BitBoard::gen(pieces, 'R'),
            wq: BitBoard::gen(pieces, 'Q'),
            wk: BitBoard::gen(pieces, 'K'),
            bp: BitBoard::gen(pieces, 'p'),
            bn: BitBoard::gen(pieces, 'n'),
            bb: BitBoard::gen(pieces, 'b'),
            br: BitBoard::gen(pieces, 'r'),
            bq: BitBoard::gen(pieces, 'q'),
            bk: BitBoard::gen(pieces, 'k'),
        }
    }
}

pub const UNIVERSAL: BitBoard = BitBoard(18446744073709551615);
pub const NOT_A_FILE: BitBoard = BitBoard(18374403900871474942);
pub const NOT_H_FILE: BitBoard = BitBoard(9187201950435737471);
pub const NOT_HG_FILE: BitBoard = BitBoard(4557430888798830399);
pub const NOT_AB_FILE: BitBoard = BitBoard(18229723555195321596);
pub const EMPTY: BitBoard = BitBoard(0);

// relevant bishop occupancy bit count for each square
pub const BISHOP_RELEVANT_BITS: [u8; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 6,
];

// relevant rook occupancy bit count for each square
pub const ROOK_RELEVANT_BITS: [u8; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 12, 11, 11, 11, 11, 11, 11, 12,
];

// convert piece list to bitboard
pub fn convert(game_status: &mut GameStatus) {
    let positions: BitPos = BitPos::new(&mut game_status.pieces);
    println!();

    println!("Black pawns: ");
    positions.bp.print();

    println!("White pawns:");
    positions.wp.print();

    let mut block = BitBoard::empty();
    block.set_bit(Square::F4);
    block.print();
    let b = bishop_atk_mask(Square::E4, block);
    b.print();

    let r = rook_atk_mask(Square::E4, block);
    r.print();

    for rank in 0..8 {
        for file in 0..8 {
            let square = rank * 8 + file;
            print!("{}, ", bishop_atk_mask(match_u32_to_sq(square), EMPTY).count_bits());
        }
        println!();
    }

    for rank in 0..8 {
        for file in 0..8 {
            let square = rank * 8 + file;
            print!("{}, ", rook_atk_mask(match_u32_to_sq(square), EMPTY).count_bits());
        }
        println!();
    }
}

// set occupancies
pub fn set_occupancy(index: u32, bits_in_mask: u32, mut attack_mask: BitBoard) -> BitBoard {
    let mut occupancy = 0;

    for count in 0..bits_in_mask {
        let sq = match_u32_to_sq(attack_mask.get_ls1b());

        attack_mask.0 |= BitBoard::from_sq(sq).0;
        attack_mask.pop_bit(sq);

        if index & (1 << count) != 0 {
            occupancy |= BitBoard::from_sq(sq).0;
        }
    }
    BitBoard(occupancy)
}

impl BitBoard {
    // generate bitboard from square
    pub fn from_sq(square: Square) -> Self {
        Self(1 << square as u8)
    }
    // init empty bitboard
    pub fn empty() -> Self {
        Self(0)
    }
    // check if bitboard is empty (u64 = 0)
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
    // check if equal
    pub fn eq(&self, rhs: Self) -> bool {
        self.0 == rhs.0
    }
    // set bit at given square (0 -> 1)
    pub fn set_bit(&mut self, square: Square) {
        self.0 |= BitBoard::from_sq(square).0;
    }
    // toggle bit at given square
    pub fn toggle_bit(&mut self, square: Square) {
        self.0 ^= BitBoard::from_sq(square).0;
    }
    // get bit at given square
    pub fn get_bit(&self, square: Square) -> bool {
        self.0 & BitBoard::from_sq(square).0 != 0
    }
    // pop bit at given square
    pub fn pop_bit(&mut self, square: Square) {
        self.0 &= !BitBoard::from_sq(square).0;

        if self.get_bit(square) {
            self.0 ^= BitBoard::from_sq(square).0;
        } else {
            self.0 = 0;
        }
    }
    // generate bitboard from piece list
    pub fn gen(pieces: &mut [Option<Piece>; 64], compare: char) -> Self {
        let mut bin_str = String::new();
        // reverse pieces array
        pieces.reverse();

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
    // print bitboard
    pub fn print(&self) {
        println!();
        println!("Hex: {:x}", self.0);
        println!("Value: {}", &self.0);
        println!();
        let mut x = 8;
        for rank in 0..8 {
            print!("\x1b[34m{}\x1b[0m  ", x);
            x -= 1;
            for file in 0..8 {
                let square = rank * 8 + file;
                if self.0 & (1 << square) >= 1 {
                    print!("\x1b[1m1\x1b[0m ");
                } else {
                    print!("\x1b[38;5;8m0\x1b[0m ");
                }
            }
            println!();
        }
        println!();
        println!("\x1b[34m   a b c d e f g h\x1b[0m");
        println!();
    }

    // rotate 180
    pub fn rot_180(&mut self) {
        let h1: BitBoard = BitBoard(0x5555555555555555);
        let h2: BitBoard = BitBoard(0x3333333333333333);
        let h4: BitBoard = BitBoard(0x0F0F0F0F0F0F0F0F);
        let v1: BitBoard = BitBoard(0x00FF00FF00FF00FF);
        let v2: BitBoard = BitBoard(0x0000FFFF0000FFFF);

        self.0 = ((self.0 >> 1) & h1.0) | ((self.0 & h1.0) << 1);
        self.0 = ((self.0 >> 2) & h2.0) | ((self.0 & h2.0) << 2);
        self.0 = ((self.0 >> 4) & h4.0) | ((self.0 & h4.0) << 4);
        self.0 = ((self.0 >> 8) & v1.0) | ((self.0 & v1.0) << 8);
        self.0 = ((self.0 >> 16) & v2.0) | ((self.0 & v2.0) << 16);
        self.0 = (self.0 >> 32) | (self.0 << 32);
    }

    // flip methods
    pub fn flip_v(&mut self) {
        let k1: BitBoard = BitBoard(0x00FF00FF00FF00FF);
        let k2: BitBoard = BitBoard(0x0000FFFF0000FFFF);
        self.0 = ((self.0 >> 8) & k1.0) | ((self.0 & k1.0) << 8);
        self.0 = ((self.0 >> 16) & k2.0) | ((self.0 & k2.0) << 16);
        self.0 = (self.0 >> 32) | (self.0 << 32);
    }
    pub fn flip_h(&mut self) {
        let k1: BitBoard = BitBoard(0x5555555555555555);
        let k2: BitBoard = BitBoard(0x3333333333333333);
        let k4: BitBoard = BitBoard(0x0f0f0f0f0f0f0f0f);
        self.0 = ((self.0 >> 1) & k1.0) | ((self.0 & k1.0) << 1);
        self.0 = ((self.0 >> 2) & k2.0) | ((self.0 & k2.0) << 2);
        self.0 = ((self.0 >> 4) & k4.0) | ((self.0 & k4.0) << 4);
    }
    // count bits within the bitboard
    pub fn count_bits(&self) -> u32 {
        self.0.count_ones()
    }
    // get least significant bit index
    pub fn get_ls1b(&self) -> u32 {
        if self.0 != 0 {
            self.0.trailing_zeros()
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{bitboard::BitBoard, movegen::*, Colour, Square};
    #[test]
    fn check_atk_lookup() {
        let p = pawn_atk_lookup(Square::E4, Colour::Black);
        assert_eq!(p.0, 43980465111040);

        let n = knight_atk_lookup(Square::E4);
        assert_eq!(n.0, 11333767002587136);

        let k = king_atk_lookup(Square::E4);
        assert_eq!(k.0, 61745389371392);

        let mut block = BitBoard::empty();
        block.set_bit(Square::G2);
        block.set_bit(Square::D5);
        block.set_bit(Square::E3);
        block.set_bit(Square::B4);
        let b = bishop_atk_mask(Square::E4, block);
        assert_eq!(b.0, 163299467632607232);

        let r = rook_atk_mask(Square::E4, block);
        assert_eq!(r.0, 18614657749008);

        assert_eq!(r.count_bits(), 11);
        assert_eq!(r.get_ls1b(), 4);
    }
}
