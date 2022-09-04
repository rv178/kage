use crate::GameStatus;

pub fn print_board(game_state: &GameStatus) {
    let mut symbol_vec: Vec<char> = Vec::new();

    for i in 0..game_state.pieces.len() {
        if game_state.pieces[i] == None {
            symbol_vec.push(' ');
        } else {
            symbol_vec.push(game_state.pieces[i].as_ref().unwrap().symbol);
        }
    }

    fn vec_to_2d_vec(vec: Vec<char>) -> Vec<Vec<char>> {
        let mut vec_2d: Vec<Vec<char>> = Vec::new();
        let mut vec_1d: Vec<char> = Vec::new();
        for item in vec {
            vec_1d.push(item);
            if vec_1d.len() == 8 {
                vec_2d.push(vec_1d);
                vec_1d = Vec::new();
            }
        }
        vec_2d
    }

    let board = vec_to_2d_vec(symbol_vec);

    fn print_board_vec(array: &[Vec<char>]) -> String {
        let mut x = 9;
        println!("+---+---+---+---+---+---+---+---+");
        let mut buf = String::new();
        for (_y, row) in array.iter().enumerate() {
            for (_x, col) in row.iter().enumerate() {
                let p_info = format!("| {} ", col);
                buf.push_str(&p_info);
            }
            x -= 1;

            let ranks = format!("| {} \n", x);
            buf.push_str(&ranks);

            buf.push_str("+---+---+---+---+---+---+---+---+\n");
        }
        buf
    }

    print!("{}", print_board_vec(&board));
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
            print!("{} ", i);
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
