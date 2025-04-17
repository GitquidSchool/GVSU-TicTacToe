use std::fmt;

// Cell enum represents each cell on the board.
// either Empty, X, or O.
 #[derive(Clone, PartialEq)]
pub enum Cell {
    Empty(usize),
    X,
    O,
}

// The Board struct holds the current state of the game board.
// Contains one cell for each position.
pub struct Board {
    pub cells: Vec<Cell>,
}

// Board methods to interact with the board.
impl Board {
    // Create board with 9 Empty cells labeled 1 through 9.
    pub fn new() -> Self {
        let cells = (1..=9).map(Cell::Empty).collect();
        Self { cells }
    }

    // Get a reference to the cell at a given index.
    pub fn get_cell(&self, index: usize) -> &Cell {
        &self.cells[index]
    }

    // Update the cell at the given index with a new symbol.
     pub fn set_cell(&mut self, index: usize, symbol: Cell) {
        self.cells[index] = symbol;
    }
}

pub fn check_game_over(board: &Board) -> Option<String> {
    // These are the winning combinations of indices (rows, cols, diagonals)
    let winning_combos = [
        [0, 1, 2], [3, 4, 5], [6, 7, 8], // Rows
        [0, 3, 6], [1, 4, 7], [2, 5, 8], // Columns
        [0, 4, 8], [2, 4, 6],            // Diagonals
    ];

    // Loop over each combo and check if all 3 cells match
    for combo in winning_combos {
        let a = board.get_cell(combo[0]);
        let b = board.get_cell(combo[1]);
        let c = board.get_cell(combo[2]);

        if a == b && b == c {
            return match a {
                Cell::X => Some("Player wins!".to_string()),
                Cell::O => Some("Computer wins!".to_string()),
                _ => None,
            };
        }
    }

    // If no winner, check if all cells are filled (tie)
    if board.cells.iter().all(|c| !matches!(c, Cell::Empty(_))) {
        return Some("It's a tie!".to_string());
    }

    // Game is not over yet
    None
}

// Implement the Display trait so we can print Cells with {}
impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Empty(n) => write!(f, "{}", n), // Show position number for Empty cells
            Cell::X => write!(f, "X"),
            Cell::O => write!(f, "O"),
        }
    }
}


