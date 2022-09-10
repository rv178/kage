use crate::{Colour, GameStatus, Piece};
use std::process::exit;

#[macro_export]
macro_rules! fen_log {
    ($($arg:tt)*) => {
        println!("\x1b[32mCranium (fen):\x1b[0m {}", format!($($arg)*));
    };
}

pub fn return_state(fen: &str) -> GameStatus {
    parse_fen(GameStatus::default_gamestatus(), fen)
}

fn parse_fen(def: GameStatus, input: &str) -> GameStatus {
    let mut fen = input.split_whitespace();
    let mut state: GameStatus = def;

    let pieces: [Option<Piece>; 64] = pieces(fen.next().unwrap());

    let colour: Colour = active_side(fen.next().unwrap());

    if colour == Colour::Undefined {
        fen_log!("Invalid FEN string: Failed to parse active colour.");
        exit(1);
    }

    let castling_id: [bool; 4] = castling_ability(fen.next().unwrap());

    let en_passant = en_passant(fen.next().unwrap());
    let half_move_clock: u32 = halfmove_clock(fen.next().unwrap());
    let full_move_count: u32 = fullmove_count(fen.next().unwrap());

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

// rather atrocious, but it works
fn en_passant(input: &str) -> Option<Vec<String>> {
    if input.chars().all(char::is_whitespace) | input.contains('-') {
        None
    } else {
        let chars = input.chars().collect::<Vec<char>>();

        if chars.len() % 2 == 0 {
            let mut ep_vec = Vec::new();

            for i in 0..chars.len() {
                if i % 2 == 0 {
                    let valid_chars = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
                    let mut invalid = true;

                    for c in valid_chars.iter() {
                        if chars[i] == *c {
                            invalid = false;
                        }
                    }

                    if i < chars.len() - 1 {
                        if !chars[i].is_alphabetic() | !chars[i + 1].is_numeric() {
                            fen_log!("Invalid FEN string: Failed to parse en passant square.");
                            exit(1);
                        }

                        if chars[i + 1]
                            .to_digit(10)
                            .expect("Could not parse character to digit (en passant)")
                            > 8
                        {
                            invalid = true;
                        }
                    }

                    if invalid {
                        fen_log!("Invalid FEN string: Failed to parse en passant square.");
                        exit(1);
                    }

                    ep_vec.push(format!("{}{}", chars[i], chars[i + 1]));
                }
            }

            Some(ep_vec)
        } else {
            fen_log!("Invalid FEN string: Failed to parse en passant square.");
            exit(1);
        }
    }
}

fn halfmove_clock(input: &str) -> u32 {
    let mut halfmove_clock = String::new();
    for c in input.chars() {
        if c.is_ascii_digit() {
            halfmove_clock.push(c);
        }
    }
    halfmove_clock.parse::<u32>().unwrap()
}

fn fullmove_count(input: &str) -> u32 {
    let mut fullmove_clock = String::new();
    for c in input.chars() {
        if c.is_ascii_digit() {
            fullmove_clock.push(c);
        }
    }
    fullmove_clock.parse::<u32>().unwrap()
}

// https://github.com/ucarion/fen/blob/master/src/lib.rs#L139-L173

fn pieces(input: &str) -> [Option<Piece>; 64] {
    let mut placement = [None; 64];
    let lines: Vec<&str> = input.split('/').collect();

    if lines.len() != 8 {
        fen_log!("Invalid FEN string: Failed to parse placement.");
        exit(1);
    }

    for (rank, pieces) in lines.iter().enumerate() {
        let mut file: usize = 0;

        for piece_char in pieces.chars() {
            match piece_char.to_digit(10) {
                Some(n) => {
                    file += n as usize;
                    if file > 8 {
                        fen_log!("Invalid FEN string: Failed to parse placement.");
                        exit(1);
                    }
                }
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
