use crate::{bitboard::*, movegen::*, utils::match_u32_to_sq};
use movegen::{king, knight, pawn};

pub mod bitboard;
pub mod fen;
pub mod movegen;
pub mod utils;

// piece enum, eg. black bishop [kind: Bishop, colour: Black, symbol: 'b']
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Piece {
    pub kind: Kind,
    pub colour: Colour,
    pub symbol: char,
}

// piece type
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Kind {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

// colour / side to move
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Colour {
    Black,
    White,
    Undefined,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameStatus {
    pub pieces: [Option<Piece>; 64],
    pub side_to_move: Colour,
    pub castling_id: [bool; 4],
    pub en_passant: Option<Vec<Square>>,
    pub half_move_clock: u16,
    pub full_move_count: u16,
}

impl GameStatus {
    fn default_gamestatus() -> GameStatus {
        GameStatus {
            pieces: [None; 64],
            side_to_move: Colour::White,
            castling_id: [true, true, true, true],
            en_passant: None,
            half_move_clock: 0,
            full_move_count: 1,
        }
    }
}

impl Piece {
    fn from_char(piece_char: char) -> Option<Piece> {
        let (colour, kind) = match piece_char {
            'P' => (Colour::White, Kind::Pawn),
            'N' => (Colour::White, Kind::Knight),
            'B' => (Colour::White, Kind::Bishop),
            'R' => (Colour::White, Kind::Rook),
            'Q' => (Colour::White, Kind::Queen),
            'K' => (Colour::White, Kind::King),
            'p' => (Colour::Black, Kind::Pawn),
            'n' => (Colour::Black, Kind::Knight),
            'b' => (Colour::Black, Kind::Bishop),
            'r' => (Colour::Black, Kind::Rook),
            'q' => (Colour::Black, Kind::Queen),
            'k' => (Colour::Black, Kind::King),
            _ => return None,
        };

        Some(Piece { colour, kind, symbol: piece_char })
    }
}

// to represent squares
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Square {
    A8, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1,
}

// return all pieces on the board except one of a particular piece type
pub fn all_other_pieces(pieces: [[BitBoard; 6]; 2], sub: BitBoard) -> BitBoard {
    let mut all_other_pieces = BitBoard::empty();
    // union with all pieces on the board
    all_other_pieces.0 |= occupancies(pieces, Colour::Undefined).0;
    // subtract intersection of all pieces with sub (aka the pieces we want to remove)
    all_other_pieces.0 -= all_other_pieces.0 & sub.0;
    all_other_pieces
}

pub fn get_attacks(side: Colour, pieces: [[BitBoard; 6]; 2]) -> BitBoard {
    let pawns = pieces[side as usize][0];
    let knights = pieces[side as usize][1];
    let bishops = pieces[side as usize][2];
    let rooks = pieces[side as usize][3];
    let queen = pieces[side as usize][4];
    let king = pieces[side as usize][5];

    // we need to put all_other_pieces(pieces, board_for_the_piece) as the occupancy mask
    let pawn_attacks = pawn::all(pawns, side);
    let knight_attacks = knight::all(knights, BitBoard::empty());
    let bishop_attacks = bishop_bb(bishops, all_other_pieces(pieces, bishops));
    let rook_attacks = rook_bb(rooks, all_other_pieces(pieces, rooks));
    let king_attacks = king::all(king, BitBoard::empty());
    let queen_attacks = queen_bb(queen, all_other_pieces(pieces, queen));

    BitBoard(
        pawn_attacks.0
            | knight_attacks.0
            | bishop_attacks.0
            | rook_attacks.0
            | king_attacks.0
            | queen_attacks.0,
    )
}

pub fn get_all_attacks(pieces: [[BitBoard; 6]; 2]) -> BitBoard {
    let white_attacks = get_attacks(Colour::White, pieces);
    let black_attacks = get_attacks(Colour::Black, pieces);

    BitBoard(white_attacks.0 | black_attacks.0)
}

impl Square {
    // check if square is attacked
    pub fn is_attacked(&self, side: Colour, pieces: [[BitBoard; 6]; 2]) -> bool {
        //println!("Square: {:?}, Side: {:?}", self, side);
        let occ = BitBoard::from_sq(*self);
        // return true or false depending on if the square is attacked (check if it intersects with
        // bitboard containing all attacks)
        get_attacks(side, pieces).0 & occ.0 != 0
    }
}

pub fn init() {
    let mut game_state = fen::return_state("3r4/1b6/3r4/R2P4/8/8/2k5/3R4 w - - 0 0");
    //let mut game_state = fen::return_state(fen::START_POS);
    fen::print_all(&game_state);
    println!();
    let pieces = convert(&mut game_state.pieces);
    print_bb_pieces(pieces, true);

    get_attacks(Colour::White, pieces).print();
    get_attacks(Colour::Black, pieces).print();
    get_all_attacks(pieces).print();
}
