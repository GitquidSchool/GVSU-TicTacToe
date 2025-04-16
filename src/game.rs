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


