pub mod bitboard;
pub mod fen;
//pub mod movegen;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Piece {
    pub kind: Kind,
    pub colour: Colour,
    pub symbol: char,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Kind {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Colour {
    White,
    Black,
    Undefined,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameStatus {
    pub pieces: [Option<Piece>; 64],
    pub side_to_move: Colour,
    pub castling_id: [bool; 4],
    pub en_passant: Option<Vec<String>>,
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
pub enum Square {
    H1, G1, F1, E1, D1, C1, B1, A1,
    H2, G2, F2, E2, D2, C2, B2, A2,
    H3, G3, F3, E3, D3, C3, B3, A3,
    H4, G4, F4, E4, D4, C4, B4, A4,
    H5, G5, F5, E5, D5, C5, B5, A5,
    H6, G6, F6, E6, D6, C6, B6, A6,
    H7, G7, F7, E7, D7, C7, B7, A7,
    H8, G8, F8, E8, D8, C8, B8, A8,
}
