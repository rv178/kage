use crate::{utils::match_u32_to_sq, Colour};
use crate::{Piece, Square};
use std::process::exit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitBoard(pub u64);

pub const UNIVERSAL: BitBoard = BitBoard(18446744073709551615);
pub const NOT_A_FILE: BitBoard = BitBoard(18374403900871474942);
pub const NOT_H_FILE: BitBoard = BitBoard(9187201950435737471);
pub const NOT_HG_FILE: BitBoard = BitBoard(4557430888798830399);
pub const NOT_AB_FILE: BitBoard = BitBoard(18229723555195321596);
pub const EMPTY: BitBoard = BitBoard(0);

// all ranks
pub const RANKS: [BitBoard; 8] = [
    BitBoard(255),                  // 8th rank
    BitBoard(65280),                // 7th rank
    BitBoard(16711680),             // 6th rank
    BitBoard(4278190080),           // 5th rank
    BitBoard(1095216660480),        // 4th rank
    BitBoard(280375465082880),      // 3rd rank
    BitBoard(71776119061217280),    // 2nd rank
    BitBoard(18374686479671623680), // 1st rank
];

// all files
pub const FILES: [BitBoard; 8] = [
    BitBoard(72340172838076673),   // a file
    BitBoard(144680345676153346),  // b file
    BitBoard(289360691352306692),  // c file
    BitBoard(578721382704613384),  // d file
    BitBoard(1157442765409226768), // e file
    BitBoard(2314885530818453536), // f file
    BitBoard(4629771061636907072), // g file
    BitBoard(9259542123273814144), // h file
];

// diagonal masks
pub const DIAG: [BitBoard; 15] = [
    BitBoard(0x80),
    BitBoard(0x8040),
    BitBoard(0x804020),
    BitBoard(0x80402010),
    BitBoard(0x8040201008),
    BitBoard(0x804020100804),
    BitBoard(0x80402010080402),
    BitBoard(0x8040201008040201),
    BitBoard(0x4020100804020100),
    BitBoard(0x2010080402010000),
    BitBoard(0x1008040201000000),
    BitBoard(0x804020100000000),
    BitBoard(0x402010000000000),
    BitBoard(0x201000000000000),
    BitBoard(0x100000000000000),
];

// anti-diagonal masks
pub const ANTI_DIAG: [BitBoard; 15] = [
    BitBoard(0x1),
    BitBoard(0x102),
    BitBoard(0x10204),
    BitBoard(0x1020408),
    BitBoard(0x102040810),
    BitBoard(0x10204081020),
    BitBoard(0x1020408102040),
    BitBoard(0x102040810204080),
    BitBoard(0x204081020408000),
    BitBoard(0x408102040800000),
    BitBoard(0x810204080000000),
    BitBoard(0x1020408000000000),
    BitBoard(0x2040800000000000),
    BitBoard(0x4080000000000000),
    BitBoard(0x8000000000000000),
];

const UNICODE_PIECES: [&str; 12] = ["♙", "♘", "♗", "♖", "♕", "♔", "♟", "♞", "♝", "♜", "♛", "♚"];
const ASCII_PIECES: [&str; 12] = ["p", "n", "b", "r", "q", "k", "P", "N", "B", "R", "Q", "K"];

// convert piece list to array of bitboards
pub fn convert(pieces: &mut [Option<Piece>; 64]) -> [[BitBoard; 6]; 2] {
    let pieces: [[BitBoard; 6]; 2] = [
        [
            BitBoard::gen(pieces, 'p'), // black pawns
            BitBoard::gen(pieces, 'n'), // black knights
            BitBoard::gen(pieces, 'b'), // black bishops
            BitBoard::gen(pieces, 'r'), // black rooks
            BitBoard::gen(pieces, 'q'), // black queens
            BitBoard::gen(pieces, 'k'), // black king
        ],
        [
            BitBoard::gen(pieces, 'P'), // white pawns
            BitBoard::gen(pieces, 'N'), // white knights
            BitBoard::gen(pieces, 'B'), // white bishops
            BitBoard::gen(pieces, 'R'), // white rooks
            BitBoard::gen(pieces, 'Q'), // white queens
            BitBoard::gen(pieces, 'K'), // white king
        ],
    ];
    pieces
}

// get all occupancies of pieces of a particular colour
pub fn occupancies(pieces: [[BitBoard; 6]; 2], side: Colour) -> BitBoard {
    let mut occ: BitBoard = BitBoard::empty();

    match side {
        Colour::White => {
            for i in 0..6 {
                occ.0 |= pieces[1][i].0;
            }
        }
        Colour::Black => {
            for i in 0..6 {
                occ.0 |= pieces[0][i].0;
            }
        }
        Colour::Undefined => {
            for i in 0..2 {
                for j in 0..6 {
                    occ.0 |= pieces[i][j].0;
                }
            }
        }
    }

    occ
}

// print chess board from array of bitboards
pub fn print_bb_pieces(pieces: [[BitBoard; 6]; 2], unicode: bool) {
    let mut x = 8;
    for rank in 0..8 {
        x -= 1;
        print!("\x1b[34m{}\x1b[0m  ", x + 1);
        for file in 0..8 {
            let square = rank * 8 + file;
            let mut piece: Option<u8> = None;

            for (x, a) in pieces.iter().enumerate() {
                for (y, b) in a.iter().enumerate() {
                    if b.get_bit(match_u32_to_sq(square)) {
                        let piece_num = y + x * 6;
                        piece = Some(piece_num as u8);
                        break;
                    }
                }
            }

            if let Some(piece) = piece {
                if unicode {
                    print!("{} ", UNICODE_PIECES[piece as usize]);
                } else {
                    print!("{} ", ASCII_PIECES[piece as usize]);
                }
            } else {
                print!("\x1b[38;5;8m.\x1b[0m ");
            }
        }
        println!();
    }
    println!();
    println!("\x1b[34m   a b c d e f g h\x1b[0m");
    println!();
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
    pub fn is_eq(&self, rhs: Self) -> bool {
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
    pub fn gen(pieces: &[Option<Piece>; 64], compare: char) -> Self {
        let mut bin_str = String::new();
        let mut pieces_rev = *pieces;
        // reverse pieces array
        pieces_rev.reverse();

        for piece in pieces_rev {
            if let Some(piece) = piece {
                if piece.symbol != compare {
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

    // generate bitboard of all pieces of one colour from piece list
    pub fn gen_colour(pieces: &mut [Option<Piece>; 64], colour: Colour) -> Self {
        let mut bin_str = String::new();
        // reverse pieces array
        //pieces.reverse();

        for piece in pieces {
            if let Some(piece) = piece {
                match colour {
                    Colour::White => {
                        if !(piece.symbol.is_lowercase()) {
                            bin_str.push('0');
                        } else {
                            bin_str.push('1');
                        }
                    }
                    Colour::Black => {
                        if !(piece.symbol.is_uppercase()) {
                            bin_str.push('0');
                        } else {
                            bin_str.push('1');
                        }
                    }
                    Colour::Undefined => exit(1),
                }
            } else {
                bin_str.push('0');
            }
        }

        // convert binary string to decimal (u64)
        Self(u64::from_str_radix(&bin_str, 2).unwrap())
    }
    // from string
    pub fn from_string(bin_str: String) -> Self {
        Self(u64::from_str_radix(&bin_str, 2).unwrap().reverse_bits())
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
        let p = pawn::lookup(Square::E4, Colour::Black);
        p.print();
        assert_eq!(p.0, 43980465111040);

        let n = knight(Square::E4);
        n.print();
        assert_eq!(n.0, 11333767002587136);

        let k = king(Square::E4);
        k.print();
        assert_eq!(k.0, 61745389371392);

        let mut block = BitBoard::empty();
        block.set_bit(Square::G2);
        block.set_bit(Square::D5);
        block.set_bit(Square::E3);
        block.set_bit(Square::B4);

        let r = rook(Square::E4, block);
        r.print();
        assert_eq!(r.0, 18614657749008);

        assert_eq!(r.count_bits(), 11);
    }
}
