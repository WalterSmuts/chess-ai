use std::io;

use chess::Board;
use chess::MoveGen;
use chess::ChessMove;
use chess::Color;

use rand::Rng;

pub trait Player {
    fn get_move(&self ,board: &Board) -> ChessMove;
}

pub struct ConsolePlayer {}
pub struct RandomPlayer {}

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

impl Player for RandomPlayer {
    fn get_move(&self, board: &Board) -> ChessMove {
        let mut moves = MoveGen::new_legal(&board);
        let mut rng = rand::thread_rng();
        let m = moves.nth(rng.gen_range(0, moves.len())).unwrap();
        return m;
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
    println!("{}'s turn to move.\n", if board.side_to_move() == Color::White {"White"} else {"Black"});
}
