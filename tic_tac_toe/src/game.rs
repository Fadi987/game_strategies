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
use std::fmt;

/// Represents the turn of the current player
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameTurn {
    TurnX,
    TurnO,
}

/// Represents the state of the game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Ongoing,
    XWon,
    OWon,
    Tie,
}

/// Represents the game objects. Interally, it keeps track of:
/// - the current board state
/// - the turn of the current player
/// - the state of the game (i.e, Ongoing, X won, O won, tie)

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Game {
    board: board::Board,
    turn: GameTurn,
    state: GameState,
}

/// Represents the possible reasons when failing to mark a board cell
#[derive(Debug, PartialEq, Eq)]
pub enum GamePlayError {
    MarkError(board::BoardMarkError),
    GameIsOver,
}

impl Game {
    /// Initializes a new `Game` object
    pub fn new() -> Self {
        Game {
            board: board::Board::new(),
            turn: GameTurn::TurnX,
            state: GameState::Ongoing,
        }
    }

    /// Plays one turn of Tic-Tac-Toe as the current player by marking cell at location (`row_index`, `col_index`).
    /// Returns an `Err` if:
    /// - location is out-of-bounds, or
    /// - cell played is non-empty, or
    /// - game is terminated (not `Ongoing`)
    pub fn play(&mut self, row_index: usize, col_index: usize) -> Result<(), GamePlayError> {
        match self.state {
            GameState::Ongoing => match self.turn {
                GameTurn::TurnX => {
                    if let Err(e) = self.board.mark(board::Cell::X, row_index, col_index) {
                        return Err(GamePlayError::MarkError(e));
                    }

                    self.update_state();
                    self.turn = GameTurn::TurnO;
                    Ok(())
                }
                GameTurn::TurnO => {
                    if let Err(e) = self.board.mark(board::Cell::O, row_index, col_index) {
                        return Err(GamePlayError::MarkError(e));
                    }

                    self.update_state();
                    self.turn = GameTurn::TurnX;
                    Ok(())
                }
            },
            _ => Err(GamePlayError::GameIsOver),
        }
    }

    /// Returns a copy of the game state after the move (row_index, col_index) has been played
    pub fn get_played(&self, row_index: usize, col_index: usize) -> Result<Self, GamePlayError> {
        let mut cloned_game = (*self).clone();
        cloned_game.play(row_index, col_index)?;
        Ok(cloned_game)
    }

    /// Gets the current state of the game
    pub fn get_state(&self) -> GameState {
        self.state
    }

    /// Gets the turn of the current player
    pub fn get_turn(&self) -> GameTurn {
        self.turn
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
        if self.is_over() {
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

    /// Returns a boolean indicating whether the game is over
    pub fn is_over(&self) -> bool {
        match self.state {
            GameState::Ongoing => false,
            _ => true,
        }
    }

    // Returns a vector of possible move as (row_index, col_index).
    // List of moves is always ordered upper left -> bottom right
    pub fn get_possible_plays(&self) -> Vec<(usize, usize)> {
        if self.is_over() {
            return Vec::new();
        }

        (0..=2)
            .flat_map(|row_index| (0..=2).map(move |col_index| (row_index, col_index)))
            .filter(|&(row_index, col_index)| {
                self.board.get_cell(row_index, col_index).unwrap() == board::Cell::Empty
            })
            .collect()
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let game_state = match self.state {
            GameState::Ongoing => "Ongoing",
            GameState::XWon => "X Won",
            GameState::OWon => "O Won",
            GameState::Tie => "Tie",
        };

        let game_turn = match self.turn {
            GameTurn::TurnX => "X",
            GameTurn::TurnO => "O",
        };

        writeln!(f, "Game State: {}, Player Turn: {}", game_state, game_turn)?;
        writeln!(f, "\n\n{}", self.board)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_game() {
        let game = Game::new();
        assert_eq!(game.board, board::Board::new());
        assert_eq!(game.state, GameState::Ongoing);
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
        assert_eq!(game.state, GameState::Ongoing);
        game.play(0, 0).unwrap();
        assert_eq!(game.state, GameState::Ongoing);
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
        assert_eq!(game.state, GameState::Ongoing);

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
        assert_eq!(game.state, GameState::Ongoing);

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
        assert_eq!(game.state, GameState::Ongoing);

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
        assert_eq!(game.state, GameState::Ongoing);

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
        assert_eq!(game.state, GameState::Ongoing);

        // X at (0, 0)
        game.play(0, 0).unwrap();
        assert_eq!(game.state, GameState::Tie);
    }

    #[test]
    fn test_out_of_bound() {
        let mut game = Game::new();
        assert_eq!(
            game.play(3, 0),
            Err(GamePlayError::MarkError(board::BoardMarkError::OutOfBound))
        );

        // Ensure that state/turn states don't change after an invalid move
        assert_eq!(game.state, GameState::Ongoing);
        assert_eq!(game.turn, GameTurn::TurnX);
    }

    #[test]
    fn test_mark_twice() {
        let mut game = Game::new();
        game.play(2, 0).unwrap();
        assert_eq!(
            game.play(2, 0),
            Err(GamePlayError::MarkError(
                board::BoardMarkError::NonEmptyCell
            ))
        );

        // Ensure that state/turn states don't change after an invalid move
        assert_eq!(game.state, GameState::Ongoing);
        assert_eq!(game.turn, GameTurn::TurnO);
    }

    #[test]
    fn test_play_game_over() {
        let mut game = Game::new();
        // X at (0, 0)
        game.play(0, 0).unwrap();
        // O at (1, 0)
        game.play(1, 0).unwrap();
        // X at (0, 1)
        game.play(0, 1).unwrap();
        // O at (1, 1)
        game.play(1, 1).unwrap();
        // X at (0, 2) -> X won
        game.play(0, 2).unwrap();

        // Playing an empty cell is invalid after game is over
        assert_eq!(game.play(2, 0), Err(GamePlayError::GameIsOver));

        // Ensure that state states don't change after an invalid move
        assert_eq!(game.state, GameState::XWon);
    }

    #[test]
    fn test_possible_plays() {
        let mut game = Game::new();

        let possible_plays: Vec<(usize, usize)> = (0..=2)
            .flat_map(|row_index| (0..=2).map(move |col_index| (row_index, col_index)))
            .collect();

        assert_eq!(possible_plays, game.get_possible_plays());

        game.play(0, 0).unwrap();
        assert_eq!(&possible_plays[1..], game.get_possible_plays());

        game.play(2, 2).unwrap();
        assert_eq!(&possible_plays[1..=7], game.get_possible_plays());
    }

    #[test]
    fn test_get_played() {
        let mut game = Game::new();
        // X at (0, 0)
        game.play(0, 0).unwrap();
        // O at (1, 0)
        game.play(1, 0).unwrap();
        // X at (0, 1)
        game.play(0, 1).unwrap();
        // O at (1, 1)
        game.play(1, 1).unwrap();
        // X at (0, 2) -> X won (in clone only)
        let game_clone = game.get_played(0, 2).unwrap();

        // Original game state is unchanged, only clone
        assert_eq!(game.state, GameState::Ongoing);
        assert_eq!(game_clone.state, GameState::XWon);
    }
}
