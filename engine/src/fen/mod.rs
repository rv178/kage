use crate::GameStatus;

pub mod display;
pub mod parser;

pub fn default() -> GameStatus {
    let default_gs =
        parser::return_state("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    default_gs
}
