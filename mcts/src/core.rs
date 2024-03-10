//! //! Contains functionality for core MCTS (Monte Carlo Tree Search)

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc;
use tic_tac_toe::game;

/// Represents a node in the Monte Carlo tree. Includes
/// - game state
/// - number of wins
/// - number of visits
/// - children which are the possible game states reachable from current state
/// - parent which is the game state we reached current game state from
struct MCTS {
    game_state: game::Game,
    parent: Option<rc::Weak<RefCell<MCTS>>>,
    children: Vec<MCTS>,
    wins: u32,
    visits: u32,
}

impl MCTS {
    /// Returns a newly created MCTS node under a shared reference
    fn new() -> rc::Rc<RefCell<MCTS>> {
        rc::Rc::new(RefCell::new(MCTS {
            game_state: game::Game::new(),
            parent: None,
            children: Vec::new(),
            wins: 0,
            visits: 0,
        }))
    }

    /// Creates a root MCTS node:
    /// - newly initialized game state
    /// - 0 wins
    /// - 0 visits
    /// - no parent
    /// - no children
    fn add_child_with_state(parent: rc::Rc<RefCell<MCTS>>, game_state: game::Game) {
        let child = MCTS {
            game_state,
            wins: 0,
            visits: 0,
            children: Vec::new(),
            parent: Some(rc::Rc::downgrade(&parent)),
        };

        (*parent).borrow_mut().children.push(child);
    }

    // Navigate from the current node until a leaf node is reaced based on UCT (Upper Confidence Bound for Trees) policy
    // fn select_node(&self) -> &Self {}

    fn expand_node(node: rc::Rc<RefCell<MCTS>>) {
        let child_games: Vec<game::Game> = (*node)
            .borrow()
            .game_state
            .get_possible_plays()
            .iter()
            .map(|&(row_index, col_index)| {
                (*node)
                    .borrow()
                    .game_state
                    .get_played(row_index, col_index)
                    .unwrap()
            })
            .collect();

        for game_state in child_games {
            MCTS::add_child_with_state(rc::Rc::clone(&node), game_state);
        }
    }

    // fn simulate_playout(&self) -> game::GameState {}

    // fn back_propagate(node: &MCTS, game_result: game::GameState) -> {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_node() {
        let root = MCTS::new();
        MCTS::expand_node(rc::Rc::clone(&root));

        // Check number of children is correct (first move has 9 possible choices)
        assert_eq!((*root).borrow().children.len(), 9);

        for node in (*root).borrow().children.iter() {
            // Assert move has been made
            assert_eq!(node.game_state.get_possible_plays().len(), 8);

            // Assert game is not over (Tic-Tac-Toe cannot end in one move)
            assert_eq!(node.game_state.get_state(), game::GameState::Ongoing);

            // Assert turn has been switched
            assert_eq!(node.game_state.get_turn(), game::GameTurn::TurnO);
        }

        let child_set: HashSet<game::Game> = (*root)
            .borrow()
            .children
            .iter()
            .map(|node| node.game_state.clone())
            .collect();

        // Make sure the child game states are unique/different (the boards in this case)
        assert_eq!(child_set.len(), 9);
    }
}
