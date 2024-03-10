use std::rc::Weak;
use tic_tac_toe::game;

struct MCTS {
    game_state: game::Game,
    parent: Option<Weak<MCTS>>,
    children: Vec<MCTS>,
    wins: u32,
    visits: u32,
}

impl MCTS {
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

    // fn expand_node(&self) -> Self {}

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
