use chess::Board;
use chess::BoardStatus;
use chess::Color;

use player::Player;

mod player;

fn main() {
    let mut board = Board::default();
    let white = player::ConsolePlayer{};
    let black = player::RandomPlayer{};
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
