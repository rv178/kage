fn main() {
    let board = vec![vec![' '; 8]; 8];

    fn get_elements(array: &Vec<Vec<char>>) -> String {
        println!("+---+---+---+---+---+---+---+---+");
        let mut buf = String::new();
        for (_y, row) in array.iter().enumerate() {
            for (_x, col) in row.iter().enumerate() {
                let p_info = format!("| {} ", col);
                buf.push_str(&p_info);
            }
            buf.push_str("|\n");
            buf.push_str("+---+---+---+---+---+---+---+---+\n");
        }
        buf
    }
    let data = get_elements(&board);
    println!("{}", data);
    println!("Fen: ");
}
