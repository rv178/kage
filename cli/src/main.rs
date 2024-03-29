use engine::bitboard::*;
use engine::*;
use engine::{fen, fen_log};
use std::{cmp::Ordering, env, process::exit};

#[macro_export]
macro_rules! main_log {
    ($($arg:tt)*) => {
        println!("\x1b[32mMain:\x1b[0m {}", format!($($arg)*));
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len().cmp(&1) {
        Ordering::Equal => {
            init();
        }
        Ordering::Greater => match args[1].as_str() {
            "-h" | "--help" => {
                help();
            }
            "-f" | "--fen" => {
                if args.len() == 3 {
                    let fen = &args[2];

                    let mut game_state = fen::return_state(fen);
                    fen::print_all(&game_state);
                    println!();
                    let pieces = convert(&mut game_state.pieces);
                    print_bb_pieces(pieces, true);
                } else {
                    fen_log!("Error: missing FEN string");
                }
            }
            "-d" | "--default" => {
                fen::print_all(&fen::default());
            }
            _ => {
                main_log!("Invalid option '{}'.", args[1]);
                exit(1);
            }
        },
        Ordering::Less => {
            exit(1);
        }
    }
}

fn help() {
    let help_msg = format!(
        "\x1b[32m\x1b[1mKage \x1b[0m {}
    Chess engine written in Rust.

\x1b[33mUSAGE:\x1b[0m
    kage \x1b[32m[OPTIONS]\x1b[0m

\x1b[33mOPTIONS:\x1b[0m
    \x1b[32m-h, --help\x1b[0m
        Show this help message.
    \x1b[32m-f, --fen <FEN_STRING>\x1b[0m
        Parse FEN string.

\x1b[33mEXAMPLES:\x1b[0m
    kage --fen \"rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2 \"

Link: \x1b[4m\x1b[34mhttps://github.com/rv178/kage\x1b[0m",
        env!("CARGO_PKG_VERSION")
    );
    println!("{}", help_msg);
}
