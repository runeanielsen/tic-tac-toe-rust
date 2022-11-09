#![warn(clippy::all, clippy::pedantic)]

mod board;

use std::io;

use board::{Board, Symbol};

use crate::board::PlayerMoveError;

#[derive(Debug, PartialEq, Eq)]
pub enum PlayerInputParseError {
    InvalidFormat(String),
}

fn parse_player_move(player_move: &str) -> Result<[usize; 2], PlayerInputParseError> {
    let positions = player_move.split(',').map(str::trim).collect::<Vec<_>>();

    if positions.len() != 2 {
        return Err(PlayerInputParseError::InvalidFormat(String::from(
            "Invalid format",
        )));
    }

    for position in &positions {
        if position.len() != 1 {
            return Err(PlayerInputParseError::InvalidFormat(String::from(
                "Invalid format",
            )));
        }
    }

    let x = match positions[0].chars().next().unwrap().to_digit(10) {
        Some(n) => n,
        None => {
            return Err(PlayerInputParseError::InvalidFormat(String::from(
                "Invalid format",
            )))
        }
    };

    let y = match positions[1].chars().next().unwrap().to_digit(10) {
        Some(n) => n,
        None => {
            return Err(PlayerInputParseError::InvalidFormat(String::from(
                "Invalid format.",
            )))
        }
    };

    Ok([x.try_into().unwrap(), y.try_into().unwrap()])
}

fn start_game() {
    let mut player_turn = Symbol::Plus;
    let mut board = Board::new();

    loop {
        println!("\nThe current board state is:\n\n{}\n", board);

        let player = match player_turn {
            Symbol::Plus => "Player 1",
            Symbol::Circle => "Player 2",
            Symbol::Empty => {
                panic!("Empty is not a valid player turn. Something is not right.")
            }
        };

        println!("{}, please do your move.", player);

        let mut player_input = String::new();
        io::stdin()
            .read_line(&mut player_input)
            .expect("Failed to read line.");

        let player_move = match parse_player_move(&player_input) {
            Ok(parsed_move) => parsed_move,
            Err(error) => match error {
                PlayerInputParseError::InvalidFormat(x) => {
                    eprintln!("{} {} please try again!", x, player);
                    continue;
                }
            },
        };

        match board.is_valid_move(player_move) {
            Ok(_) => {}
            Err(err) => match err {
                PlayerMoveError::FilledPosition(msg) | PlayerMoveError::OutsideBoard(msg) => {
                    eprintln!("{} {} please try again!", msg, player);
                    continue;
                }
            },
        };

        board.place(player_turn, player_move);

        if board.winner().is_some() {
            println!("The winner is: {}", player);
            break;
        }

        player_turn = match player_turn {
            Symbol::Plus => Symbol::Circle,
            Symbol::Circle => Symbol::Plus,
            Symbol::Empty => panic!("Invalid player turn."),
        }
    }
}

fn main() {
    start_game();
    println!("Thanks for playing!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_player_moves_test() {
        let valid_moves = [
            ("0,1", [0, 1]),
            ("2,2", [2, 2]),
            ("3,0", [3, 0]),
            ("0,0", [0, 0]),
            ("0 , 0", [0, 0]),
            ("3, 0", [3, 0]),
            ("3 ,0", [3, 0]),
            ("2 ,0", [2, 0]),
        ];

        for (player_move, expected) in valid_moves {
            assert_eq!(parse_player_move(player_move).unwrap(), expected);
        }
    }

    #[test]
    fn invalid_player_moves_test() {
        let invalid_moves = [",1", "2,", "30", "sdfss,sfsdfs", "a,b", ""];

        for invalid_move in invalid_moves {
            assert_eq!(
                parse_player_move(invalid_move),
                Err(PlayerInputParseError::InvalidFormat(
                    "Invalid format".to_string()
                ))
            );
        }
    }
}
