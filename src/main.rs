use chess::Board;

fn main() {
    let board = Board::default();
    print_board(board);
}

fn print_board(board: Board) {
    let fen = format!("{}", board);

    println!("{}", "-".repeat(33));
    print!("{}","|");
    for c in fen.chars() {
        if c.is_numeric() {
            print!("{}", "   |".repeat(c.to_digit(10).unwrap() as usize));
        } else if c == '/'{
            print!("\n");
            println!("{}", "-".repeat(33));
            print!("|");
        } else if c != ' '{
            print!(" {} |", c);
        } else {
            break;
        }
    }
    print!("\n");
    println!("{}", "-".repeat(33));
}
