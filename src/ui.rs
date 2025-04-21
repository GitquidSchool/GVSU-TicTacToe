use std::io::{self, Write};
use crate::game::{Board, Cell};

// Display the board in a clean 3x3 grid format
pub fn print_board(board: &Board) {
    println!("\n-------------"); // Top border

    for i in 0..9 {
        // Print each cell using its Display implementation
        print!("| {} ", board.get_cell(i));

        // Every third cell ends the row
        if i % 3 == 2 {
            println!("|\n-------------");
        }
    }
}

// Ask the player for a move and return a valid index (0–8)
pub fn prompt_player_move(board: &Board) -> usize {
    loop {
        // Ask the user for input
        print!("Your move (1-9): ");
        io::stdout().flush().expect("Failed to flush stdout");

        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Try converting input into a number
        if let Ok(pos) = input.trim().parse::<usize>() {
            // Ensure it's within the allowed range
            if pos >= 1 && pos <= 9 {
                let idx = pos - 1;

                // Only return if the spot is empty
                if let Cell::Empty(_) = board.get_cell(idx) {
                    return idx;
                } else {
                    println!("That spot is already taken. Try again.");
                }
            } else {
                println!("Please enter a number between 1 and 9.");
            }
        } else {
            println!("Invalid input. Enter a number between 1 and 9.");
        }
    }
}
