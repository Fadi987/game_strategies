use std::io;
use std::io::Write;
use tic_tac_toe::board;
use tic_tac_toe::game;

fn parse_input(input: &String) -> Result<(usize, usize), (&'static str)> {
    let parts: Vec<&str> = input.split(',').collect();

    if parts.len() != 2 {
        return Err("Number of comma separated non-negative numbers must be 2.");
    }

    match (
        parts[0].trim().parse::<usize>(),
        parts[1].trim().parse::<usize>(),
    ) {
        (Ok(row_index), Ok(col_index)) => Ok((row_index, col_index)),
        _ => Err("Must enter valid non-negative numbers separated by a comma."),
    }
}
fn main() {
    let mut game = game::Game::new();
    loop {
        println!("{}", game);

        if game.is_over() {
            println!("Game Over!");
            break;
        }

        let player = match game.get_turn() {
            game::GameTurn::TurnX => "X",
            game::GameTurn::TurnO => "O",
        };

        print!(
            "Select cell for player {} in format row_index, col_index: ",
            player
        );
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read Tic-Tac-Toe move.");

        match parse_input(&input) {
            Ok((row_index, col_index)) => match game.play(row_index, col_index) {
                Err(game::GamePlayError::MarkError(board::BoardMarkError::OutOfBound)) => {
                    println!("Index out of bound. Try again.")
                }
                Err(game::GamePlayError::MarkError(board::BoardMarkError::NonEmptyCell)) => {
                    println!("Cannot mark a non empty cell. Try again.")
                }
                Ok(()) => {
                    continue;
                }
                _ => {
                    panic!("Should not get here!");
                }
            },
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}
