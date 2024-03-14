//! Contains functionality for core MCTN (Monte Carlo Tree Search)

use rand::Rng;
use std::cell::RefCell;
use std::rc;
use tic_tac_toe::game;

/// Represents a node in the Monte Carlo tree. Includes
/// - game state
/// - parent which is the game state we reached current game state from
/// - children which are the possible game states reachable from current state
/// - game move played in parent state to reach current node (None if parent is None)
/// - number of wins
/// - number of visits
struct MCTN {
    game: game::Game,
    parent: Option<rc::Weak<RefCell<MCTN>>>,
    children: Vec<rc::Rc<RefCell<MCTN>>>,
    move_from_parent: Option<(usize, usize)>,
    wins: f64,
    visits: f64,
}

impl MCTN {
    /// Returns a newly created MCTN (Monte Carlo Tree Node) starting from `game_state` under a shared pointer
    pub fn new(game_state: &game::Game) -> rc::Rc<RefCell<MCTN>> {
        rc::Rc::new(RefCell::new(MCTN {
            game: game_state.clone(),
            parent: None,
            move_from_parent: None,
            children: Vec::new(),
            wins: 0.0,
            visits: 0.0,
        }))
    }

    /// Compute UCT (Upper Confidence Bound for Trees) score
    fn uct(parent_visits: f64, child_wins: f64, child_visits: f64) -> f64 {
        let win_rate = child_wins / child_visits;

        // TODO: think about the choice of the constant C for exploration/exploitation trade-off
        win_rate + (2.0 as f64).sqrt() * (parent_visits.ln() / child_visits).sqrt()
    }

    // Navigate from the current node until a leaf node is reaced based on UCT (Upper Confidence Bound for Trees) policy
    fn select_node(node: rc::Rc<RefCell<MCTN>>) -> rc::Rc<RefCell<MCTN>> {
        let mut max_uct_child: Option<rc::Rc<RefCell<MCTN>>> = None;
        let mut max_uct = 0.0;

        for child in (*node).borrow().children.iter() {
            let uct = MCTN::uct(
                (*node).borrow().visits,
                (**child).borrow().wins,
                (**child).borrow().visits,
            );

            if uct > max_uct {
                max_uct = uct;
                max_uct_child = Some(rc::Rc::clone(child));
            }
        }

        match max_uct_child {
            Some(child) => MCTN::select_node(child),
            None => node,
        }
    }

    /// Starting from the parent game, plays a move (`play_row_index`, `play_col_index`), and adds
    /// the new game state as a child
    fn play(parent: rc::Rc<RefCell<MCTN>>, play_row_index: usize, play_col_index: usize) {
        let child = rc::Rc::new(RefCell::new(MCTN {
            game: (*parent)
                .borrow()
                .game
                .get_played(play_row_index, play_col_index)
                .unwrap(),
            wins: 0.0,
            visits: 0.0,
            children: Vec::new(),
            move_from_parent: Some((play_row_index, play_col_index)),
            parent: Some(rc::Rc::downgrade(&parent)),
        }));

        (*parent).borrow_mut().children.push(child);
    }

    /// Starting from current MCTN, adds children corresponding to all possible next moves
    /// If game is already over, it is a no-op
    fn expand_node(node: rc::Rc<RefCell<MCTN>>) {
        if (*node).borrow().children.len() > 0 {
            panic!("Cannot expand a non-leaf node!");
        }

        let possible_plays = (*node).borrow().game.get_possible_plays();

        for (play_row_index, play_col_index) in possible_plays {
            MCTN::play(rc::Rc::clone(&node), play_row_index, play_col_index);
        }
    }

    /// Simulate a random play starting from game state in `node` until game is over
    fn simulate_playout(node: rc::Rc<RefCell<MCTN>>) -> game::GameState {
        let mut cloned_game = (*node).borrow().game.clone();
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

    // Starting form leaf node, refresh the state of wins/vists up the tree until root node is reached
    fn backpropagate(node: rc::Rc<RefCell<MCTN>>, game_result: game::GameState) {
        let node_player = (*node).borrow().game.get_turn();

        match game_result {
            game::GameState::XWon => match node_player {
                game::GameTurn::TurnX => {
                    (*node).borrow_mut().visits += 1.0;
                }
                game::GameTurn::TurnO => {
                    (*node).borrow_mut().visits += 1.0;
                    (*node).borrow_mut().wins += 1.0;
                }
            },
            game::GameState::OWon => match node_player {
                game::GameTurn::TurnX => {
                    (*node).borrow_mut().visits += 1.0;
                    (*node).borrow_mut().wins += 1.0;
                }
                game::GameTurn::TurnO => {
                    (*node).borrow_mut().visits += 1.0;
                }
            },
            game::GameState::Tie => match node_player {
                game::GameTurn::TurnX => {
                    (*node).borrow_mut().visits += 1.0;
                    (*node).borrow_mut().wins += 0.5;
                }
                game::GameTurn::TurnO => {
                    (*node).borrow_mut().visits += 1.0;
                    (*node).borrow_mut().wins += 0.5;
                }
            },
            _ => panic!("Cannot back propagate result other than XWon, OWon, Tie"),
        }

        match &(*node).borrow().parent {
            Some(parent) => {
                MCTN::backpropagate(rc::Rc::clone(&parent.upgrade().unwrap()), game_result)
            }
            None => {}
        }
    }

    // Perform one round of an MCTS (Monte Carlo Tree Search) update. This includes:
    // 1- selecting a leaf node starting from root according to UCT policy
    // 2- expanding leaf node to include its children of possible new moves
    // 3- simulating a random playout starting from each of the children
    // 4- backpropagating game results of random playouts from each new children up to the root node
    fn mcts_update(root: rc::Rc<RefCell<MCTN>>) {
        let leaf = MCTN::select_node(rc::Rc::clone(&root));
        MCTN::expand_node(rc::Rc::clone(&leaf));

        let children: Vec<rc::Rc<RefCell<MCTN>>> = (*leaf).borrow().children.clone();

        for child in children {
            let game_result = MCTN::simulate_playout(rc::Rc::clone(&child));
            MCTN::backpropagate(rc::Rc::clone(&child), game_result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_node() {
        let root = MCTN::new(&game::Game::new());
        MCTN::expand_node(rc::Rc::clone(&root));

        // Check number of children is correct (first move has 9 possible choices)
        assert_eq!((*root).borrow().children.len(), 9);

        for node in (*root).borrow().children.iter() {
            // Assert move has been made
            assert_eq!((**node).borrow().game.get_possible_plays().len(), 8);

            // Assert game is not over (Tic-Tac-Toe cannot end in one move)
            assert_eq!((**node).borrow().game.get_state(), game::GameState::Ongoing);

            // Assert turn has been switched
            assert_eq!((**node).borrow().game.get_turn(), game::GameTurn::TurnO);
        }

        use std::collections;
        let child_set: collections::HashSet<game::Game> = (*root)
            .borrow()
            .children
            .iter()
            .map(|node| (**node).borrow().game.clone())
            .collect();

        // Make sure the child game states (the boards in this case) are unique/different
        assert_eq!(child_set.len(), 9);
    }

    #[test]
    fn test_select_node() {
        let root = MCTN::new(&game::Game::new());

        // X at (0, 0) added as a first possibility child
        MCTN::play(rc::Rc::clone(&root), 0, 0);

        // X at (2, 2) added as a second possibility child
        MCTN::play(rc::Rc::clone(&root), 2, 2);

        // At this point, we have a root node with two children at level 1

        assert_eq!((*root).borrow().children.len(), 2);

        // Pick a child arbitrarily
        let a_child = rc::Rc::clone((*root).borrow().children.iter().next().unwrap());

        // Backpropagate XWon from the chosen child. This should increase UCT score for that child
        MCTN::backpropagate(rc::Rc::clone(&a_child), game::GameState::XWon);

        let selected_child = MCTN::select_node(rc::Rc::clone(&root));

        // Make sure we select the child with high UCT
        assert_eq!(
            (*selected_child).borrow().game == (*a_child).borrow().game,
            true
        );
    }

    #[test]
    fn test_backpropagate_xwon() {
        // Start with new game (empty board)
        let root = MCTN::new(&game::Game::new());

        // X at (0, 0) added as a child
        MCTN::play(rc::Rc::clone(&root), 0, 0);

        assert_eq!((*root).borrow().children.len(), 1);
        let child = rc::Rc::clone((*root).borrow().children.iter().next().unwrap());

        // Propagate state XWon up from child to parent
        MCTN::backpropagate(rc::Rc::clone(&child), game::GameState::XWon);

        // Make sure child increased both wins and vists
        assert_eq!(((*child).borrow().wins - 1.0).abs() < 1e-7, true);
        assert_eq!(((*child).borrow().visits - 1.0).abs() < 1e-7, true);

        // Make sure parent only increased vists
        assert_eq!(((*root).borrow().wins - 0.0).abs() < 1e-7, true);
        assert_eq!(((*root).borrow().visits - 1.0).abs() < 1e-7, true);
    }

    #[test]
    fn test_backpropagate_owon() {
        // Start with new game (empty board)
        let root = MCTN::new(&game::Game::new());

        // X at (0, 0) added as a child
        MCTN::play(rc::Rc::clone(&root), 0, 0);

        assert_eq!((*root).borrow().children.len(), 1);
        let child = rc::Rc::clone((*root).borrow().children.iter().next().unwrap());

        // Propagate state OWn up from child to parent
        MCTN::backpropagate(rc::Rc::clone(&child), game::GameState::OWon);

        // Make sure child increased only increased vists
        assert_eq!(((*child).borrow().wins - 0.0).abs() < 1e-7, true);
        assert_eq!(((*child).borrow().visits - 1.0).abs() < 1e-7, true);

        // Make sure parent increased both wins and vists
        assert_eq!(((*root).borrow().wins - 1.0).abs() < 1e-7, true);
        assert_eq!(((*root).borrow().visits - 1.0).abs() < 1e-7, true);
    }

    #[test]
    fn test_backpropagate_tie() {
        // Start with new game (empty board)
        let root = MCTN::new(&game::Game::new());

        // X at (0, 0) added as a child
        MCTN::play(rc::Rc::clone(&root), 0, 0);

        assert_eq!((*root).borrow().children.len(), 1);
        let child = rc::Rc::clone((*root).borrow().children.iter().next().unwrap());

        // Propagate state Tie up from child to parent
        MCTN::backpropagate(rc::Rc::clone(&child), game::GameState::Tie);

        // Make sure child increased vists by 1 and wins by 0.5
        assert_eq!(((*child).borrow().wins - 0.5).abs() < 1e-7, true);
        assert_eq!(((*child).borrow().visits - 1.0).abs() < 1e-7, true);

        // Make sure parent increased vists by 1 and wins by 0.5
        assert_eq!(((*root).borrow().wins - 0.5).abs() < 1e-7, true);
        assert_eq!(((*root).borrow().visits - 1.0).abs() < 1e-7, true);
    }

    #[test]
    fn test_backpropagate_2_levels() {
        // Start with new game (empty board)
        let root = MCTN::new(&game::Game::new());

        // X at (0, 0) added as a child
        MCTN::play(rc::Rc::clone(&root), 0, 0);

        assert_eq!((*root).borrow().children.len(), 1);

        let child_level_1 = rc::Rc::clone((*root).borrow().children.iter().next().unwrap());

        // O at (1, 1) added as a child second level
        MCTN::play(rc::Rc::clone(&child_level_1), 1, 1);

        assert_eq!((*child_level_1).borrow().children.len(), 1);

        let child_level_2 =
            rc::Rc::clone((*&child_level_1).borrow().children.iter().next().unwrap());

        // Propagate state XWon two levels up the tree from child to parent
        MCTN::backpropagate(rc::Rc::clone(&child_level_2), game::GameState::XWon);

        // Make sure 2nd child increased only vists by 1
        assert_eq!(((*child_level_2).borrow().wins - 0.0).abs() < 1e-7, true);
        assert_eq!(((*child_level_2).borrow().visits - 1.0).abs() < 1e-7, true);

        // Make sure 1st child increased both vists and wins by 1
        assert_eq!(((*child_level_1).borrow().wins - 1.0).abs() < 1e-7, true);
        assert_eq!(((*child_level_1).borrow().visits - 1.0).abs() < 1e-7, true);

        // Make sure root increased only vists by 1
        assert_eq!(((*root).borrow().wins - 0.0).abs() < 1e-7, true);
        assert_eq!(((*root).borrow().visits - 1.0).abs() < 1e-7, true);
    }
}
