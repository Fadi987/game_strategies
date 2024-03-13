//! Contains functionality for core MCTS (Monte Carlo Tree Search)

use rand::Rng;
use std::cell::RefCell;
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
    children: Vec<rc::Rc<RefCell<MCTS>>>,
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

    /// Adds a child MCTS node to parent with game state `game_state`
    fn add_child_with_state(parent: rc::Rc<RefCell<MCTS>>, game_state: game::Game) {
        let child = rc::Rc::new(RefCell::new(MCTS {
            game_state,
            wins: 0,
            visits: 0,
            children: Vec::new(),
            parent: Some(rc::Rc::downgrade(&parent)),
        }));

        (*parent).borrow_mut().children.push(child);
    }

    fn uct(parent_visits: f64, child_wins: f64, child_visits: f64) -> f64 {
        let win_rate = child_wins / child_visits;

        win_rate + (2.0 as f64).sqrt() * (parent_visits.ln() / child_visits).sqrt()
    }

    // Navigate from the current node until a leaf node is reaced based on UCT (Upper Confidence Bound for Trees) policy
    fn select_node(node: rc::Rc<RefCell<MCTS>>) -> rc::Rc<RefCell<MCTS>> {
        let mut max_uct_child: Option<rc::Rc<RefCell<MCTS>>> = None;
        let mut max_uct = 0.0;

        for child in (*node).borrow().children.iter() {
            let uct = MCTS::uct(
                (*node).borrow().visits as f64,
                (**child).borrow().wins as f64,
                (**child).borrow().visits as f64,
            );

            if uct > max_uct {
                max_uct = uct;
                max_uct_child = Some(rc::Rc::clone(child));
            }
        }

        match max_uct_child {
            Some(child) => MCTS::select_node(child),
            None => node,
        }
    }

    /// Starting from current MCTS node, adds children corresponding to all possible next moves
    /// If game is already over, it is a no-op
    fn expand_node(node: rc::Rc<RefCell<MCTS>>) {
        if (*node).borrow().children.len() > 0 {
            panic!("Cannot expand a non-leaf node!");
        }

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

    /// Simulate a random play starting from game state in `node` until game is over
    fn simulate_playout(node: rc::Rc<RefCell<MCTS>>) -> game::GameState {
        let mut cloned_game = (*node).borrow().game_state.clone();
        let mut rng = rand::thread_rng();

        while !cloned_game.is_over() {
            let possible_plays = cloned_game.get_possible_plays();
            let (rnd_row_idx, rnd_col_idx) = possible_plays[rng.gen_range(0..possible_plays.len())];
            cloned_game.play(rnd_row_idx, rnd_col_idx).unwrap();
        }

        assert_eq!(cloned_game.get_possible_plays().len(), 0);
        assert_ne!(cloned_game.get_state(), game::GameState::Ongoing);

        cloned_game.get_state()
    }

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
            assert_eq!((**node).borrow().game_state.get_possible_plays().len(), 8);

            // Assert game is not over (Tic-Tac-Toe cannot end in one move)
            assert_eq!(
                (**node).borrow().game_state.get_state(),
                game::GameState::Ongoing
            );

            // Assert turn has been switched
            assert_eq!(
                (**node).borrow().game_state.get_turn(),
                game::GameTurn::TurnO
            );
        }

        use std::collections;
        let child_set: collections::HashSet<game::Game> = (*root)
            .borrow()
            .children
            .iter()
            .map(|node| (**node).borrow().game_state.clone())
            .collect();

        // Make sure the child game states (the boards in this case) are unique/different
        assert_eq!(child_set.len(), 9);
    }
}
