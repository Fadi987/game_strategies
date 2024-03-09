//! Contains functionality for constructing and playing a Tic-Tac-Toe game
//! ## Examples
//!
//! ```
//! use tic_tac_toe::game;
//!
//! let mut game = game::Game::new();
//! game.play(0, 0); // Player X plays at position (0, 0)
//! game.play(1, 1); // Player O plays at position (1, 1)
//! ```

use crate::board;

/// Represents the turn of the current player
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameTurn {
    TurnX,
    TurnO,
}

/// Represents the state of the game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    OnGoing,
    XWon,
    OWon,
    Tie,
}

/// Represents the game objects. Interally, it keeps track of:
/// - the current board state
/// - the turn of the current player
/// - the state of the game (i.e, ongoing, X won, O won, tie)
pub struct Game {
    board: board::Board,
    turn: GameTurn,
    state: GameState,
}

impl Game {
    /// Initializes a new `Game` object
    pub fn new() -> Self {
        Game {
            board: board::Board::new(),
            turn: GameTurn::TurnX,
            state: GameState::OnGoing,
        }
    }

    /// Plays one turn of Tic-Tac-Toe as the current player by marking cell at location (`row_index`, `col_index`).
    /// Returns an `Err` if:
    /// - location is out-of-bounds, or
    /// - cell played is non-empty, or
    /// - game is terminated (not `OnGoing`)
    pub fn play(&mut self, row_index: usize, col_index: usize) -> Result<(), &'static str> {
        match self.state {
            GameState::OnGoing => match self.turn {
                GameTurn::TurnX => {
                    self.board.mark(board::Cell::X, row_index, col_index)?;
                    self.update_state();
                    self.turn = GameTurn::TurnO;
                    Ok(())
                }
                GameTurn::TurnO => {
                    self.board.mark(board::Cell::O, row_index, col_index)?;
                    self.update_state();
                    self.turn = GameTurn::TurnX;
                    Ok(())
                }
            },
            _ => Err("Cannot play a terminated game."),
        }
    }

    /// Gets the current state of the game
    pub fn get_state(&self) -> GameState {
        self.state
    }

    fn check_win(&self, path: [(usize, usize); 3]) -> (u32, u32) {
        let mut x_streak = 0;
        let mut o_streak = 0;

        for coord in path {
            match self.board.get_cell(coord.0, coord.1).unwrap() {
                board::Cell::X => x_streak += 1,
                board::Cell::O => o_streak += 1,
                _ => {}
            }
        }

        (x_streak, o_streak)
    }

    fn update_state(&mut self) {
        if self.state != GameState::OnGoing {
            panic!("Cannot update state when game is terminated!")
        }

        let paths: [[(usize, usize); 3]; 8] = [
            // Row win paths
            [(0, 0), (0, 1), (0, 2)],
            [(1, 0), (1, 1), (1, 2)],
            [(2, 0), (2, 1), (2, 2)],
            // Column win paths
            [(0, 0), (1, 0), (2, 0)],
            [(0, 1), (1, 1), (2, 1)],
            [(0, 2), (1, 2), (2, 2)],
            // Diagonal win paths
            [(0, 0), (1, 1), (2, 2)],
            [(0, 2), (1, 1), (2, 0)],
        ];

        let mut found_empty = false;
        for path in paths {
            match self.check_win(path) {
                (3, _) => {
                    self.state = GameState::XWon;
                    return;
                }
                (_, 3) => {
                    self.state = GameState::OWon;
                    return;
                }
                (x_streak, o_streak) if x_streak + o_streak < 3 => found_empty = true,
                _ => {}
            }
        }

        if !found_empty {
            self.state = GameState::Tie;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_game() {
        let game = Game::new();
        assert_eq!(game.board, board::Board::new());
        assert_eq!(game.state, GameState::OnGoing);
        assert_eq!(game.turn, GameTurn::TurnX);
    }

    #[test]
    fn test_turn_switch() {
        let mut game = Game::new();
        assert_eq!(game.turn, GameTurn::TurnX);
        game.play(0, 0).unwrap();
        assert_eq!(game.turn, GameTurn::TurnO);
    }

    #[test]
    fn test_state_ongoing() {
        let mut game = Game::new();
        assert_eq!(game.state, GameState::OnGoing);
        game.play(0, 0).unwrap();
        assert_eq!(game.state, GameState::OnGoing);
    }

    #[test]
    fn test_x_won_horizontal() {
        let mut game = Game::new();
        // X at (0, 0)
        game.play(0, 0).unwrap();
        // O at (1, 0)
        game.play(1, 0).unwrap();
        // X at (0, 1)
        game.play(0, 1).unwrap();
        // O at (1, 1)
        game.play(1, 1).unwrap();
        assert_eq!(game.state, GameState::OnGoing);

        // X at (0, 2) -> X won
        game.play(0, 2).unwrap();
        assert_eq!(game.state, GameState::XWon);
    }

    #[test]
    fn test_o_won_vertical() {
        let mut game = Game::new();
        // X at (0, 0)
        game.play(0, 0).unwrap();
        // O at (0, 1)
        game.play(0, 1).unwrap();
        // X at (0, 2)
        game.play(0, 2).unwrap();
        // O at (1, 1)
        game.play(1, 1).unwrap();
        // X at (1, 0)
        game.play(1, 0).unwrap();
        assert_eq!(game.state, GameState::OnGoing);

        // O at (2, 1) -> O won
        game.play(2, 1).unwrap();
        assert_eq!(game.state, GameState::OWon);
    }

    #[test]
    fn test_x_won_first_diagonal() {
        let mut game = Game::new();
        // X at (0, 0)
        game.play(0, 0).unwrap();
        // O at (0, 1)
        game.play(0, 1).unwrap();
        // X at (1, 1)
        game.play(1, 1).unwrap();
        // O at (0, 2)
        game.play(0, 2).unwrap();
        assert_eq!(game.state, GameState::OnGoing);

        // X at (2, 2) -> X Won
        game.play(2, 2).unwrap();
        assert_eq!(game.state, GameState::XWon);
    }

    #[test]
    fn test_x_won_second_diagonal() {
        let mut game = Game::new();
        // X at (0, 2)
        game.play(0, 2).unwrap();
        // O at (0, 0)
        game.play(0, 0).unwrap();
        // X at (1, 1)
        game.play(1, 1).unwrap();
        // O at (0, 1)
        game.play(0, 1).unwrap();
        assert_eq!(game.state, GameState::OnGoing);

        // X at (2, 0) -> X Won
        game.play(2, 0).unwrap();
        assert_eq!(game.state, GameState::XWon);
    }

    #[test]
    fn test_tie() {
        let mut game = Game::new();
        // X at (2, 0)
        game.play(2, 0).unwrap();
        // O at (1, 1)
        game.play(1, 1).unwrap();
        // X at (2, 2)
        game.play(2, 2).unwrap();
        // O at (2, 1)
        game.play(2, 1).unwrap();
        // X at (1, 2)
        game.play(1, 2).unwrap();
        // O at (1, 0)
        game.play(1, 0).unwrap();
        // X at (0, 1)
        game.play(0, 1).unwrap();
        // O at (0, 2)
        game.play(0, 2).unwrap();
        assert_eq!(game.state, GameState::OnGoing);

        // X at (0, 0)
        game.play(0, 0).unwrap();
        assert_eq!(game.state, GameState::Tie);
    }
}
