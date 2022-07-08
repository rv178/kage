use colored::Colorize;
use std::env;
use std::process::exit;

mod debug;
mod fen;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "-h" | "--help" => {
            help();
        }
        "-f" | "--fen" => {
            if args.len() == 3 {
                let fen = &args[2];
                main_fen(fen.to_string());
            } else {
                println!("Error: missing FEN string");
            }
        }
        _ => {
            println!("Invalid option '{}'.", args[1].to_string());
            exit(1);
        }
    }
}

fn main_fen(fen_str: String) {
    let game_state = fen::return_state(&fen_str);
    debug::print_board(&game_state);
    debug::print_side_to_move(&game_state);
    debug::print_castling_ability(&game_state);
    debug::print_en_passant_squares(&game_state);
    debug::print_full_moves(&game_state);
    debug::print_half_moves(&game_state);
}

fn help() {
    print!("\n");
    print!("{}", "Cranium ".green().bold());
    println!("{}", env!("CARGO_PKG_VERSION"));
    println!("  Chess engine written in Rust.");
    print!("\n");
    println!("{}", "USAGE:".yellow());
    println!("  cranium {}", "[OPTIONS]".green());
    print!("\n");
    println!("{}", "OPTIONS:".yellow());
    println!("  {}", "-h, --help".green());
    println!("      Show this help message.");
    println!("  {}", "-f, --fen <FEN_STRING>".green());
    println!("      Parse FEN string.");
    print!("\n");
    println!("{}", "EXAMPLES:".yellow());
    println!("  cranium --fen \"rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/PNBQKB1R w KQkq - 1 2\"");
}
