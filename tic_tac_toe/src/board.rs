//! Contains functionality for manipulating a Tic-Tac-Toe board

use std::fmt;

/// Represents a Tic-Tac-Toe Cell
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    X,
    O,
    Empty,
}

/// Represents a 3x3 Tic-Tac-Toe board
#[derive(Debug, PartialEq, Eq)]
pub struct Board {
    cells: [[Cell; 3]; 3],
}

impl Board {
    /// Constructs a new Tic-Tac-Toe `Board`
    pub fn new() -> Self {
        Board {
            cells: [[Cell::Empty; 3]; 3],
        }
    }

    /// Marks the `Board` object with cell `Cell` at location (`row_index`, `col_index`). Returns an `Err` if:
    /// - location marked is out-of-bounds, or
    /// - the chosen cell is non-empty
    pub fn mark(
        &mut self,
        mark: Cell,
        row_index: usize,
        col_index: usize,
    ) -> Result<(), &'static str> {
        match self
            .cells
            .get_mut(row_index)
            .and_then(|r| r.get_mut(col_index))
        {
            Some(cell) => match cell {
                Cell::Empty => {
                    *cell = mark;
                    Ok(())
                }
                _ => Err("Cannot mark a non-empty cell."),
            },
            None => Err("Index out-of-bound."),
        }
    }

    /// Returns `Cell` at location (`row_index`, `col_index`), or an `Err` if location is out-of-bound
    pub fn get_cell(&self, row_index: usize, col_index: usize) -> Result<Cell, &'static str> {
        self.cells
            .get(row_index)
            .and_then(|r| r.get(col_index).copied())
            .ok_or("Board index out of bound.")
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row_index in 0..=2 {
            for col_index in 0..=2 {
                let symbol = match self.cells[row_index][col_index] {
                    Cell::X => "X",
                    Cell::O => "O",
                    Cell::Empty => " ",
                };

                if col_index < 2 {
                    write!(f, " {} |", symbol)?;
                } else {
                    write!(f, " {} ", symbol)?;
                }
            }

            if row_index < 2 {
                writeln!(f, "\n-----------")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_board() {
        let b = Board::new();
        for row_index in 0..=2 {
            for col_index in 0..=2 {
                assert_eq!(b.cells[row_index][col_index], Cell::Empty);
            }
        }
    }

    #[test]
    fn test_mark_board() {
        let mut b = Board::new();
        b.mark(Cell::X, 0, 0).unwrap();
        assert_eq!(b.cells[0][0], Cell::X);
    }

    #[test]
    fn test_mark_board_fails_oob() {
        let mut b = Board::new();
        let result = b.mark(Cell::X, 5, 1);
        assert_eq!(result, Err("Index out-of-bound."));
    }

    #[test]
    fn test_mark_board_fails_non_empty_cell() {
        let mut b = Board::new();
        b.mark(Cell::X, 0, 0).unwrap();
        let result = b.mark(Cell::O, 0, 0);
        assert_eq!(result, Err("Cannot mark a non-empty cell."));
    }
}
