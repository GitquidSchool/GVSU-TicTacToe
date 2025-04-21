mod game; // Game logic module
mod ui;   // UI module for displaying and input

use game::{Board, Cell, check_game_over};
use ui::{print_board, prompt_player_move};
use rand::seq::IteratorRandom; // For random selection
use rand::rng;          // Random number generator
use std::process::Command;     // Used to clear the console

fn clear_screen() {
    // Try clearing the terminal for Windows and Unix systems
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

fn main() {
    let mut board = Board::new(); // Initialize game board

    loop {
        clear_screen();           // Clear terminal for better UI
        println!("Tic Tac Toe - Player (X) vs Computer (O)");
        print_board(&board);      // Show current board

        // === Player's Move ===
        let player_move = prompt_player_move(&board); // Ask player for move
        board.set_cell(player_move, Cell::X);         // Mark cell as X

        // Check for end condition after player's move
        if let Some(result) = check_game_over(&board) {
            clear_screen();
            print_board(&board);
            println!("{}", result);
            break;
        }

        // === Computer's Move ===
        let mut rng = rng(); // Create random generator

        // Collect indices of empty cells
        let empty_cells: Vec<usize> = board.cells.iter().enumerate()
            .filter_map(|(i, cell)| if matches!(cell, Cell::Empty(_)) { Some(i) } else { None })
            .collect();

        // Randomly choose one of the empty spots
        if let Some(&comp_move) = empty_cells.iter().choose(&mut rng) {
            board.set_cell(comp_move, Cell::O); // Mark cell as O
        }

        // Check for end condition after computer's move
        if let Some(result) = check_game_over(&board) {
            clear_screen();
            print_board(&board);
            println!("{}", result);
            break;
        }
    }

    println!("Game over. Thanks for playing!");
}
