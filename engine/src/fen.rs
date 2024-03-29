use crate::utils::match_u32_to_sq;
use crate::{Colour, GameStatus, Piece, Square};
use std::process::exit;

pub const EMPTY_BOARD: &str = "8/8/8/8/8/8/8/8 w - - 0 0";
pub const START_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ";
pub const TRICKY_POS: &str =
    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 ";
pub const KILLER_POS: &str = "rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1";
pub const CMK_POS: &str = "r2q1rk1/ppp2ppp/2n1bn2/2b1p3/3pP3/3P1NPP/PPP1NPB1/R1BQ1RK1 b - - 0 9 ";

#[macro_export]
macro_rules! fen_log {
    ($($arg:tt)*) => {
        println!("\x1b[32mFen:\x1b[0m {}", format!($($arg)*));
    };
}

pub fn default() -> GameStatus {
    return_state("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
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
    let half_move_clock: u16 = halfmove_clock(fen.next().unwrap());
    let full_move_count: u16 = fullmove_count(fen.next().unwrap());

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
fn en_passant(input: &str) -> Option<Vec<Square>> {
    if input.chars().all(char::is_whitespace) | input.contains('-') {
        None
    } else {
        let chars = input.chars().collect::<Vec<char>>();

        if chars.len() % 2 == 0 {
            let mut ep_vec = Vec::new();
            let mut sq;

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

                    let file = match chars[i] {
                        'a' => 0,
                        'b' => 1,
                        'c' => 2,
                        'd' => 3,
                        'e' => 4,
                        'f' => 5,
                        'g' => 6,
                        'h' => 7,
                        _ => {
                            fen_log!("Invalid FEN string: Failed to parse en passant square.");
                            exit(1);
                        }
                    };

                    let rank = match chars[i + 1] {
                        '1' => 7,
                        '2' => 6,
                        '3' => 5,
                        '4' => 4,
                        '5' => 3,
                        '6' => 2,
                        '7' => 1,
                        '8' => 0,
                        _ => {
                            fen_log!("Invalid FEN string: Failed to parse en passant square.");
                            exit(1);
                        }
                    };

                    sq = rank * 8 + file;

                    ep_vec.push(match_u32_to_sq(sq as u32));
                }
            }

            Some(ep_vec)
        } else {
            fen_log!("Invalid FEN string: Failed to parse en passant square.");
            exit(1);
        }
    }
}

fn halfmove_clock(input: &str) -> u16 {
    let mut halfmove_clock = String::new();
    for c in input.chars() {
        if c.is_ascii_digit() {
            halfmove_clock.push(c);
        }
    }
    halfmove_clock.parse::<u16>().unwrap()
}

fn fullmove_count(input: &str) -> u16 {
    let mut fullmove_clock = String::new();
    for c in input.chars() {
        if c.is_ascii_digit() {
            fullmove_clock.push(c);
        }
    }
    fullmove_clock.parse::<u16>().unwrap()
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

// display the board
pub fn print_board(game_state: &GameStatus) {
    let mut board: Vec<char> = Vec::new();

    for piece in game_state.pieces {
        if piece == None {
            board.push(' ');
        } else {
            board.push(piece.as_ref().unwrap().symbol);
        }
    }

    let mut x = 8;
    println!("+---+---+---+---+---+---+---+---+");
    for rank in 0..8 {
        x -= 1;
        for file in 0..8 {
            let square = rank * 8 + file;
            if board[square] == ' ' {
                print!("|   ");
            } else {
                print!("| {} ", board[square]);
            }
        }
        println!("| {} ", x + 1);
        println!("+---+---+---+---+---+---+---+---+");
    }

    println!("  a   b   c   d   e   f   g   h  \n");
}

pub fn print_side_to_move(game_state: &GameStatus) {
    println!("{:?} to move", game_state.side_to_move);
}

pub fn print_en_passant_squares(game_state: &GameStatus) {
    if game_state.en_passant == None {
        println!("No en passant squares.");
    } else {
        print!("Available en passant squares: ");
        for i in game_state.en_passant.as_ref().unwrap() {
            print!("{:?} ", i);
        }
        println!();
    }
}

pub fn print_castling_ability(game_state: &GameStatus) {
    if game_state.castling_id[0] {
        println!("White can castle kingside.");
    }
    if game_state.castling_id[1] {
        println!("White can castle queenside.");
    }
    if game_state.castling_id[2] {
        println!("Black can castle kingside.");
    }
    if game_state.castling_id[3] {
        println!("Black can castle queenside.");
    }
}

pub fn print_half_moves(game_state: &GameStatus) {
    println!("Half moves: {} move(s)", game_state.half_move_clock);
}

pub fn print_full_moves(game_state: &GameStatus) {
    println!("Full moves: {} move(s)", game_state.full_move_count);
}

pub fn print_all(game_state: &GameStatus) {
    print_board(game_state);
    print_side_to_move(game_state);
    print_en_passant_squares(game_state);
    print_castling_ability(game_state);
    print_half_moves(game_state);
    print_full_moves(game_state);
}
