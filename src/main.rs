use chess::Board;
use chess::MoveGen;
use chess::ChessMove;
use chess::BoardStatus;
use chess::Color;
use std::io;

trait Player {
    fn get_move(&self ,board: &Board) -> ChessMove;
}

struct ConsolePlayer {}

fn get_input(size: usize) -> usize {
    let mut input = String::new();
    println!("Choose a move from 0 to {}", size - 1);
    io::stdin().read_line(&mut input).unwrap();
    if let Ok(n) = input.trim().parse() {
        if n < size {
            return n;
        }
    }
    println!("Try again...");
    return get_input(size);
}

impl Player for ConsolePlayer {
    fn get_move(&self, board: &Board) -> ChessMove {
        print_board(board);
        let moves = MoveGen::new_legal(&board);
        let mut i = 0;
        println!("List of possible moves:");
        for m in moves.into_iter() {
            println!("{}: {}", i, m);
            i = i + 1;
        }
        let mut moves = MoveGen::new_legal(&board);
        let m = moves.nth(get_input(moves.len())).unwrap();
        return m;
    }
}

fn main() {
    let mut board = Board::default();
    let white = ConsolePlayer{};
    let black = ConsolePlayer{};
    while board.status() == BoardStatus::Ongoing {
        match board.side_to_move() {
            Color::White => board = board.make_move_new(white.get_move(&board)),
            Color::Black => board = board.make_move_new(black.get_move(&board)),
        }
    }
    match board.status() {
        BoardStatus::Checkmate => println!("Checkmate! {} wins!", if board.side_to_move() == Color::White {"White"} else{"Black"}),
        BoardStatus::Stalemate => println!("Stalemate, game is a draw."),
        BoardStatus::Ongoing   => println!("How did I end up here?"),
    }
}

fn print_board(board: &Board) {
    println!("    a   b   c   d   e   f   g   h");
    let fen = format!("{}", board);

    println!("  {}", "-".repeat(33));
    let mut i = 0;
    print!("{} ", 8-i);
    print!("|");
    for c in fen.chars() {
        if c.is_numeric() {
            print!("{}", "   |".repeat(c.to_digit(10).unwrap() as usize));
        } else if c == '/'{
            print!(" {}", 8-i);
            i = i+1;
            print!("\n");
            println!("  {}", "-".repeat(33));
            print!("{} ", 8-i);
            print!("|");
        } else if c != ' '{
            print!(" {} |", c);
        } else {
            break;
        }
    }
    print!(" {}", 8-i);
    print!("\n");
    println!("  {}", "-".repeat(33));
    println!("    a   b   c   d   e   f   g   h");
    println!("{}'s turn to move.\n", if board.side_to_move() == Color::White {"White"} else{"Black"});
}
