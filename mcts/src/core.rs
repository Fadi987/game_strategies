//! //! Contains functionality for core MCTS (Monte Carlo Tree Search)

use std::rc::Weak;
use tic_tac_toe::game;

/// Represents a node in the Monte Carlo tree. Includes
/// - game state
/// - number of wins
/// - number of visits
/// - children which are the possible game states reachable from current state
/// - parent which is the game state we reached current game state from
struct MCTS {
    game_state: game::Game,
    parent: Option<Weak<MCTS>>,
    children: Vec<MCTS>,
    wins: u32,
    visits: u32,
}

impl MCTS {
    /// Creates a root MCTS node:
    /// - newly initialized game state
    /// - 0 wins
    /// - 0 visits
    /// - no parent
    /// - no children
    pub fn new() -> Self {
        MCTS {
            game_state: game::Game::new(),
            children: Vec::new(),
            parent: None,
            wins: 0,
            visits: 0,
        }
    }

    // Navigate from the current node until a leaf node is reaced based on UCT (Upper Confidence Bound for Trees) policy
    // fn select_node(&self) -> &Self {}

    fn expand_node(&mut self) {
        for (row_index, col_index) in self.game_state.get_possible_plays() {
            self.game_state.play(row_index, col_index).unwrap();
        }
    }

    // fn simulate_playout(&self) -> game::GameState {}

    // fn back_propagate(node: &MCTS, game_result: game::GameState) -> {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, 2);
    }
}
