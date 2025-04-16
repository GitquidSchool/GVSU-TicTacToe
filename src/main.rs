mod game;

use game::{Board, Cell};

fn main() {
    let board = Board::new();

    for i in 0..9 {
        print!("| {} ", board.get_cell(i));
        if i % 3 == 2 {
            println!("|");
        }
    }
}