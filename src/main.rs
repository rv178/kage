use std::env;
use std::process::exit;

mod debug;
mod fen;

#[macro_export]
macro_rules! main_log {
    ($($arg:tt)*) => {
        println!("\x1b[32mCranium (main):\x1b[0m {}", format!($($arg)*));
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "-h" | "--help" => {
                help();
            }
            "-f" | "--fen" => {
                if args.len() == 3 {
                    let fen = &args[2];

                    let game_state = fen::return_state(fen);
                    debug::print_board(&game_state);
                    debug::print_side_to_move(&game_state);
                    debug::print_castling_ability(&game_state);
                    debug::print_en_passant_squares(&game_state);
                    debug::print_full_moves(&game_state);
                    debug::print_half_moves(&game_state);
                } else {
                    fen_log!("Error: missing FEN string");
                }
            }
            _ => {
                main_log!("Invalid option '{}'.", args[1]);
                exit(1);
            }
        }
    } else if args.len() == 1 {
        println!("\x1b[32m\x1b[1mCranium \x1b[0m {}", env!("CARGO_PKG_VERSION"));
    }
}

fn help() {
    let help_msg = format!(
        "\x1b[32m\x1b[1mCranium \x1b[0m {}
    Chess engine written in Rust.

\x1b[33mUSAGE:\x1b[0m
    cranium \x1b[32m[OPTIONS]\x1b[0m

\x1b[33mOPTIONS:\x1b[0m
    \x1b[32m-h, --help\x1b[0m
        Show this help message.
    \x1b[32m-f, --fen <FEN_STRING>\x1b[0m
        Parse FEN string.

\x1b[33mEXAMPLES:\x1b[0m
    cranium --fen \"rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/PNBQKB1R w KQkq - 1 2\"

Link: \x1b[4m\x1b[34mhttps://github.com/rv178/cranium\x1b[0m",
        env!("CARGO_PKG_VERSION")
    );
    println!("{}", help_msg);
}
