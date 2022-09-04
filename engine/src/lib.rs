pub mod bitboard;
pub mod fen;
//pub mod movegen;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Piece {
    pub kind: Kind,
    pub colour: Colour,
    pub symbol: char,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Kind {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Colour {
    White,
    Black,
    Undefined,
}

#[derive(Debug)]
pub struct GameStatus {
    pub pieces: Vec<Option<Piece>>,
    pub side_to_move: Colour,
    pub castling_id: [bool; 4],
    pub en_passant: Option<Vec<String>>,
    pub half_move_clock: u32,
    pub full_move_count: u32,
}

impl GameStatus {
    fn default_gamestatus() -> GameStatus {
        GameStatus {
            pieces: vec![None; 64],
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
        let (color, kind, symbol) = match piece_char {
            'P' => (Colour::White, Kind::Pawn, 'P'),
            'N' => (Colour::White, Kind::Knight, 'N'),
            'B' => (Colour::White, Kind::Bishop, 'B'),
            'R' => (Colour::White, Kind::Rook, 'R'),
            'Q' => (Colour::White, Kind::Queen, 'Q'),
            'K' => (Colour::White, Kind::King, 'K'),
            'p' => (Colour::Black, Kind::Pawn, 'p'),
            'n' => (Colour::Black, Kind::Knight, 'n'),
            'b' => (Colour::Black, Kind::Bishop, 'b'),
            'r' => (Colour::Black, Kind::Rook, 'r'),
            'q' => (Colour::Black, Kind::Queen, 'q'),
            'k' => (Colour::Black, Kind::King, 'k'),
            _ => return None,
        };

        Some(Piece { colour: color, kind, symbol })
    }
}

// to represent squares, will do this later
/*
#[rustfmt::skip]
pub enum Squares {
    None,
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}
*/
