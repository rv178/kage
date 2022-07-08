use regex::Regex;
use std::process::exit;

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

#[macro_export]
macro_rules! fen_log {
    ($($arg:tt)*) => {
        println!("\x1b[32mCranium (fen):\x1b[0m {}", format!($($arg)*));
    };
}

pub fn return_state(fen: &str) -> GameStatus {
    let state = parse_fen(GameStatus::default_gamestatus(), fen);
    state
}

fn parse_fen(def: GameStatus, input: &str) -> GameStatus {
    let mut fen = input.split_whitespace();
    let mut state = def;

    let pieces = pieces(fen.next().unwrap());

    let colour = active_side(fen.next().unwrap());
    if colour == Colour::Undefined {
        fen_log!("Invalid FEN string: Failed to parse active colour.");
        exit(1);
    }

    let castling_id = castling_ability(fen.next().unwrap());

    let en_passant = en_passant(fen.next().unwrap());
    let half_move_clock = halfmove_clock(fen.next().unwrap());
    let full_move_count = fullmove_count(fen.next().unwrap());

    state.pieces = pieces;
    state.side_to_move = colour;
    state.castling_id = castling_id;
    state.en_passant = en_passant;
    state.half_move_clock = half_move_clock;
    state.full_move_count = full_move_count;

    state
}

fn active_side(input: &str) -> Colour {
    match input {
        "w" => Colour::White,
        "b" => Colour::Black,
        _ => Colour::Undefined,
    }
}

fn castling_ability(input: &str) -> [bool; 4] {
    if input == "-" {
        [false, false, false, false]
    } else {
        let mut castling_id = [false; 4];
        let mut castling_id_str = input.to_string();
        castling_id_str.retain(|c| c != '-');
        for c in castling_id_str.chars() {
            match c {
                'K' => castling_id[0] = true,
                'Q' => castling_id[1] = true,
                'k' => castling_id[2] = true,
                'q' => castling_id[3] = true,
                _ => {
                    fen_log!("Invalid FEN string: Failed to parse castling ability.");
                    exit(1);
                }
            }
        }
        castling_id
    }
}

fn parse_en_passant_squares(input: &str) -> Vec<String> {
    let re = Regex::new(r"^-|[a-g]\d$").unwrap();
    if re.is_match(input) {
        let mut char_vec: Vec<String> = Vec::new();
        let mut split_vec: Vec<String> = Vec::new();
        let mut en_passant_vec: Vec<String> = Vec::new();

        let split_string = input.split(|c: char| c.is_whitespace());
        for s in split_string {
            for c in s.chars() {
                char_vec.push(c.to_string());
            }
        }
        for i in 0..char_vec.len() {
            if i % 2 == 0 {
                split_vec.push(char_vec[i].to_string());
                split_vec.push(char_vec[i + 1].to_string());
                en_passant_vec.push(split_vec.join(""));
                split_vec.clear();
            }
        }

        en_passant_vec
    } else {
        fen_log!("Invalid FEN string: Failed to parse en passant squares.");
        exit(1);
    }
}

fn en_passant(input: &str) -> Option<Vec<String>> {
    if input == "-" {
        None
    } else if input.chars().all(char::is_whitespace) {
        None
    } else {
        Some(parse_en_passant_squares(input))
    }
}

fn halfmove_clock(input: &str) -> u32 {
    let mut halfmove_clock = String::new();
    for c in input.chars() {
        if c.is_digit(10) {
            halfmove_clock.push(c);
        }
    }
    halfmove_clock.parse::<u32>().unwrap()
}

fn fullmove_count(input: &str) -> u32 {
    let mut fullmove_clock = String::new();
    for c in input.chars() {
        if c.is_digit(10) {
            fullmove_clock.push(c);
        }
    }
    fullmove_clock.parse::<u32>().unwrap()
}

// --- taken from https://github.com/ucarion/fen/blob/master/src/lib.rs ---

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

fn pieces(input: &str) -> Vec<Option<Piece>> {
    let mut placement = vec![None; 64];
    let lines: Vec<_> = input.split('/').collect();

    if lines.len() != 8 {
        fen_log!("Invalid FEN string: Failed to parse placement.");
        exit(1);
    }

    for (rank, pieces) in lines.iter().enumerate() {
        let mut file = 0;

        for piece_char in pieces.chars() {
            match piece_char.to_digit(10) {
                // This is indicating a few blanks
                Some(n) => {
                    increment_file(&mut file, n as usize, pieces);
                }

                // This is indicating a piece
                None => match Piece::from_char(piece_char) {
                    Some(piece) => {
                        placement[rank * 8 + file] = Some(piece);
                        file += 1;
                    }

                    None => {
                        fen_log!("Invalid FEN string: Failed to parse piece.");
                        exit(1);
                    }
                },
            }
        }
    }
    placement
}

fn increment_file(file: &mut usize, n: usize, _rank: &str) {
    *file += n;
    if *file > 8 {
        fen_log!("Invalid FEN string: Failed to parse placement.");
        exit(1);
    }
}
