use std::io;

use chess::Board;
use chess::MoveGen;
use chess::ChessMove;
use chess::Color;
use chess::BoardStatus;

use rand::Rng;

pub trait Player {
    fn get_move(&self ,board: &Board) -> ChessMove;
}

pub struct ConsolePlayer;
pub struct RandomPlayer;
pub struct GreedyPlayer;
pub struct TreePlayer{
    pub depth: i32,
}

fn get_input(board: &Board) -> usize {
    let moves = MoveGen::new_legal(&board);
    let mut input = String::new();
    let size = moves.len();
    println!("Choose a move from 0 to {}", size - 1);
    io::stdin().read_line(&mut input).unwrap();
    if let Ok(n) = input.trim().parse() {
        if n < size {
            return n;
        }
    }

    let mut i = 0;
    for m in moves.into_iter() {
        if format!("{}", m) == input.trim() {
            return i;
        }
        i = i + 1;
    }

    println!("Try again...");
    get_input(&board)
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
        let m = moves.nth(get_input(&board)).unwrap();
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

impl Player for GreedyPlayer {
    fn get_move(&self, board: &Board) -> ChessMove {
        let mut moves = MoveGen::new_legal(&board);
        let mut greedy_move = moves.next().unwrap();
        for m in moves.into_iter() {
            let test_board = board.make_move_new(m);
            let greedy_board = board.make_move_new(greedy_move);
            let better = match board.side_to_move() {
                Color::White => board_score(&test_board) > board_score(&greedy_board),
                Color::Black => board_score(&test_board) < board_score(&greedy_board),
            };
            if better {
                greedy_move = m;
            }
        }
        return greedy_move;
    }
}

impl Player for TreePlayer {
    fn get_move(&self, board: &Board) -> ChessMove {
        let mut moves = MoveGen::new_legal(&board);
        let simulated_player = get_player(self.depth);
        let mut greedy_move = moves.next().unwrap();
        for m in moves.into_iter() {
            let mut greedy_board = board.make_move_new(greedy_move);
            let mut test_board = board.make_move_new(m);

            if test_board.status() != BoardStatus::Checkmate {
                test_board = test_board.make_move_new(simulated_player.get_move(&test_board));
            }
            if greedy_board.status() != BoardStatus::Checkmate {
                greedy_board = greedy_board.make_move_new(simulated_player.get_move(&greedy_board));
            }
            let better = match board.side_to_move() {
                Color::White => board_score(&test_board) > board_score(&greedy_board),
                Color::Black => board_score(&test_board) < board_score(&greedy_board),
            };
            if better {
                greedy_move = m;
            }
        }
        return greedy_move;
    }
}

fn get_player(depth: i32) -> Box<dyn Player> {
    if depth == 0 {
        Box::new(GreedyPlayer)
    } else {
        Box::new(TreePlayer{depth: depth -1})
    }
}

fn board_score(board: &Board) -> i32 {
    if board.status() == BoardStatus::Checkmate {
        match board.side_to_move() {
            Color::White => return std::i32::MIN,
            Color::Black => return std::i32::MAX,
        }
    }
    let fen = format!("{}", board);
    let mut score:i32 = 0;
    for c in fen.chars() {
        match c {
            'p' => score = score - 1,
            'n' => score = score - 3,
            'b' => score = score - 3,
            'r' => score = score - 5,
            'q' => score = score - 9,
            'P' => score = score + 1,
            'N' => score = score + 3,
            'B' => score = score + 3,
            'R' => score = score + 5,
            'Q' => score = score + 9,
            ' ' => break,
            _   => (),
        }
    }
    score = score * 100 + MoveGen::new_legal(&board).len() as i32;
    score
}

pub fn print_board(board: &Board) {
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
    println!("Current board score: {}", board_score(&board));
    println!("{:?}'s turn to move.\n", board.side_to_move());
}
