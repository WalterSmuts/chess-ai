use chess::Board;
use chess::BoardStatus;
use chess::Color;

use player::Player;

mod player;

fn main() {
    let mut board = Board::default();
    let white = player::ConsolePlayer;
    let black = player::TreePlayer{depth: 2};
    let mut move_count = 0;
    while board.status() == BoardStatus::Ongoing {
        player::print_board(&board);
        if move_count > 10000 {
            break;
        }
        match board.side_to_move() {
            Color::White => board = board.make_move_new(white.get_move(&board)),
            Color::Black => board = board.make_move_new(black.get_move(&board)),
        }
        move_count = move_count + 1;
    }
    player::print_board(&board);
    match board.status() {
        BoardStatus::Checkmate => println!("Checkmate! {:?} loses in {} moves!", board.side_to_move(), move_count),
        BoardStatus::Stalemate => println!("Stalemate, game is a draw."),
        BoardStatus::Ongoing   => println!("How did I end up here?"),
    }
}
