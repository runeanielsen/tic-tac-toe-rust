use std::{fmt::Display, io};

use crate::board::{Board, PlayerMoveError, Symbol};

#[derive(Debug, PartialEq, Eq)]
enum PlayerInputParseError {
    InvalidFormat(String),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Player {
    One,
    Two,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let alias = match self {
            Player::One => "Player 1",
            Player::Two => "Player 2",
        };

        write!(f, "{}", alias)
    }
}

impl From<Player> for Symbol {
    fn from(val: Player) -> Self {
        match val {
            Player::One => Symbol::Plus,
            Player::Two => Symbol::Circle,
        }
    }
}

fn parse_player_move(player_move: &str) -> Result<[usize; 2], PlayerInputParseError> {
    let positions = player_move.split(',').map(str::trim).collect::<Vec<_>>();

    let invalid_format_error_message = "Invalid format";

    if positions.len() != 2 {
        return Err(PlayerInputParseError::InvalidFormat(String::from(
            invalid_format_error_message,
        )));
    }

    for position in &positions {
        if position.len() != 1 {
            return Err(PlayerInputParseError::InvalidFormat(String::from(
                invalid_format_error_message,
            )));
        }
    }

    let x = match positions[0].chars().next().unwrap().to_digit(10) {
        Some(n) => n,
        None => {
            return Err(PlayerInputParseError::InvalidFormat(String::from(
                invalid_format_error_message,
            )))
        }
    };

    let y = match positions[1].chars().next().unwrap().to_digit(10) {
        Some(n) => n,
        None => {
            return Err(PlayerInputParseError::InvalidFormat(String::from(
                invalid_format_error_message,
            )))
        }
    };

    Ok([x.try_into().unwrap(), y.try_into().unwrap()])
}

pub fn start() {
    let mut player_turn = Player::One;
    let mut board = Board::new();

    loop {
        println!("\nThe current board state is:\n\n{}\n", board);

        println!("{}, please do your move.", player_turn);

        let mut player_input = String::new();
        io::stdin()
            .read_line(&mut player_input)
            .expect("Failed to read line.");

        let player_move = match parse_player_move(&player_input) {
            Ok(parsed_move) => parsed_move,
            Err(error) => match error {
                PlayerInputParseError::InvalidFormat(x) => {
                    eprintln!("{} {} please try again!", x, player_turn);
                    continue;
                }
            },
        };

        match board.is_valid_move(player_move) {
            Ok(_) => {}
            Err(err) => match err {
                PlayerMoveError::FilledPosition(msg) | PlayerMoveError::OutsideBoard(msg) => {
                    eprintln!("{} {} please try again!", msg, player_turn);
                    continue;
                }
            },
        };

        board.place(player_turn.into(), player_move);

        if board.winner().is_some() {
            println!("The winner is: {}", player_turn);
            break;
        }

        player_turn = match player_turn {
            Player::One => Player::Two,
            Player::Two => Player::One,
        }
    }
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

    #[test]
    fn player_alias_for_shoutout_test() {
        assert_eq!("Player 1", format!("{}", Player::One));
        assert_eq!("Player 2", format!("{}", Player::Two));
    }
}
