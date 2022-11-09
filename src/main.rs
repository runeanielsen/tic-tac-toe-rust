#![warn(clippy::all, clippy::pedantic)]

mod board;

use std::io;

use board::{Representation, Board};

#[derive(Debug, PartialEq, Eq)]
pub enum PlayerMoveError {
    InvalidFormat(String),
    FilledPosition(String),
    OutsideBoard(String),
}

fn parse_player_move(player_move: &str) -> Result<[usize; 2], PlayerMoveError> {
    let positions = player_move.split(',').map(str::trim).collect::<Vec<_>>();

    if positions.len() != 2 {
        return Err(PlayerMoveError::InvalidFormat(String::from(
            "Invalid format",
        )));
    }

    for position in &positions {
        if position.len() != 1 {
            return Err(PlayerMoveError::InvalidFormat(String::from(
                "Invalid format",
            )));
        }
    }

    let x = match positions[0].chars().next().unwrap().to_digit(10) {
        Some(n) => n,
        None => {
            return Err(PlayerMoveError::InvalidFormat(String::from(
                "Invalid format",
            )))
        }
    };

    let y = match positions[1].chars().next().unwrap().to_digit(10) {
        Some(n) => n,
        None => {
            return Err(PlayerMoveError::InvalidFormat(String::from(
                "Invalid format.",
            )))
        }
    };

    Ok([x.try_into().unwrap(), y.try_into().unwrap()])
}

fn is_valid_move(
    player_move: [usize; 2],
    board: &Board,
) -> Result<bool, PlayerMoveError> {
    if player_move[0] > 2 || player_move[1] > 2 {
        return Err(PlayerMoveError::OutsideBoard(String::from(
            "The move is invalid because it is outside the board.",
        )));
    }

    if board.0[player_move[0]][player_move[1]] != Representation::Empty {
        return Err(PlayerMoveError::FilledPosition(String::from(
            "The position is already filled.",
        )));
    }

    Ok(true)
}

fn place_on_board(symbol: Representation, player_move: [usize; 2], board: &Board) -> Board {
    let mut board = board.clone();
    board.0[player_move[0]][player_move[1]] = symbol;
    board
}

fn find_winner(board: &Board) -> Option<Representation> {
    let board_state = board.0;

    // Check for row and column winner.
    for i in 0..board_state.len() {
        // Row winner
        if board_state[i][0] == board_state[i][1] && board_state[i][0] == board_state[i][2] {
            // Empty cannot be a winner :)
            if board_state[i][0] != Representation::Empty {
                return Some(board_state[i][0]);
            }
        }

        // Colum winner
        if board_state[0][i] == board_state[1][i] && board_state[0][i] == board_state[2][i] {
            // Empty cannot be a winner :)
            if board_state[0][i] != Representation::Empty {
                return Some(board_state[0][i]);
            }
        }
    }

    // Left to right winner
    if board_state[0][0] != Representation::Empty
        && board_state[0][0] == board_state[1][1]
        && board_state[0][0] == board_state[2][2]
    {
        return Some(board_state[0][0]);
    }

    // Right to left winner
    if board_state[0][2] != Representation::Empty
        && board_state[0][2] == board_state[1][1]
        && board_state[0][2] == board_state[2][0]
    {
        return Some(board_state[0][2]);
    }

    None
}

fn start_game() {
    let mut player_turn = Representation::Plus;
    let mut board = Board::new();

    loop {
        println!("\nThe current board state is:\n\n{}\n", board);

        let player = match player_turn {
            Representation::Plus => "Player 1",
            Representation::Circle => "Player 2",
            Representation::Empty => {
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
                PlayerMoveError::InvalidFormat(x) => {
                    eprintln!("{} {} please try again!", x, player);
                    continue;
                }
                _ => panic!("Unhandled error."),
            },
        };

        match is_valid_move(player_move, &board) {
            Ok(_) => {}
            Err(err) => match err {
                PlayerMoveError::FilledPosition(msg) | PlayerMoveError::OutsideBoard(msg) => {
                    eprintln!("{} {} please try again!", msg, player);
                    continue;
                }
                PlayerMoveError::InvalidFormat(_) => {
                    panic!("Invalid format.")
                }
            },
        };

        board = place_on_board(player_turn, player_move, &board);

        if find_winner(&board).is_some() {
            println!("The winner is: {}", player);
            break;
        }

        player_turn = match player_turn {
            Representation::Plus => Representation::Circle,
            Representation::Circle => Representation::Plus,
            Representation::Empty => panic!("Invalid player turn."),
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
    fn can_convert_from_board_symbol_to_string() {
        let assertions = [
            (Representation::Empty, "-"),
            (Representation::Plus, "+"),
            (Representation::Circle, "o"),
        ];

        for (value, expected) in assertions {
            assert_eq!(expected, Into::<&str>::into(value));
        }
    }

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
                Err(PlayerMoveError::InvalidFormat("Invalid format".to_string()))
            );
        }
    }

    #[test]
    fn place_on_board_test() {
        let mut board = Board([[Representation::Empty; 3]; 3]);
        board = place_on_board(Representation::Plus, [1, 1], &board);

        let mut expected = [[Representation::Empty; 3]; 3];
        expected[1][1] = Representation::Plus;

        (0..board.0.len()).for_each(|i| {
            for j in 0..board.0[i].len() {
                assert_eq!(board.0[i][j], expected[i][j]);
            }
        });
    }

    #[test]
    fn valid_player_move_empty_board() {
        let board = Board::new();
        let valid_moves = [
            [0, 0],
            [0, 1],
            [0, 2],
            [1, 0],
            [1, 1],
            [1, 2],
            [2, 0],
            [2, 1],
            [2, 2],
        ];

        for valid_move in valid_moves {
            assert!(is_valid_move(valid_move, &board).unwrap());
        }
    }

    #[test]
    fn valid_player_move_symbols_on_board() {
        let mut board = Board::new();
        board.0[1][1] = Representation::Plus;
        board.0[2][2] = Representation::Circle;

        let valid_moves = [[0, 0], [0, 1], [0, 2], [1, 0], [1, 2], [2, 0], [2, 1]];

        for valid_move in valid_moves {
            assert!(is_valid_move(valid_move, &board).unwrap());
        }
    }

    #[test]
    fn invalid_player_move_already_filled_slot() {
        let mut board = Board::new();
        board.0[1][1] = Representation::Plus;

        assert_eq!(
            is_valid_move([1, 1], &board),
            Err(PlayerMoveError::FilledPosition(String::from(
                "The position is already filled."
            )))
        );
    }

    #[test]
    fn invalid_player_move_outside_bounds() {
        let board = Board::new();

        let invalid_moves = [[1, 3], [3, 1], [5, 5], [100, 100]];

        for invalid_move in invalid_moves {
            assert_eq!(
                is_valid_move(invalid_move, &board),
                Err(PlayerMoveError::OutsideBoard(
                    "The move is invalid because it is outside the board.".to_string()
                ))
            );
        }
    }

    #[test]
    fn find_winner_row_winner_test() {
        let mut first_row_filled = Board([[Representation::Empty; 3]; 3]);
        first_row_filled.0[0][0] = Representation::Circle;
        first_row_filled.0[0][1] = Representation::Circle;
        first_row_filled.0[0][2] = Representation::Circle;

        let mut second_row_filled = Board([[Representation::Empty; 3]; 3]);
        second_row_filled.0[1][0] = Representation::Plus;
        second_row_filled.0[1][1] = Representation::Plus;
        second_row_filled.0[1][2] = Representation::Plus;

        let mut third_row_filled = Board([[Representation::Empty; 3]; 3]);
        third_row_filled.0[2][0] = Representation::Plus;
        third_row_filled.0[2][1] = Representation::Plus;
        third_row_filled.0[2][2] = Representation::Plus;

        assert_eq!(find_winner(&first_row_filled).unwrap(), Representation::Circle);
        assert_eq!(find_winner(&second_row_filled).unwrap(), Representation::Plus);
        assert_eq!(find_winner(&third_row_filled).unwrap(), Representation::Plus);
    }

    #[test]
    fn find_column_winner() {
        let mut first_column_filled = Board::new();
        first_column_filled.0[0][0] = Representation::Circle;
        first_column_filled.0[1][0] = Representation::Circle;
        first_column_filled.0[2][0] = Representation::Circle;

        let mut second_column_filled = Board::new();
        second_column_filled.0[0][1] = Representation::Plus;
        second_column_filled.0[1][1] = Representation::Plus;
        second_column_filled.0[2][1] = Representation::Plus;

        let mut third_column_filled = Board::new();
        third_column_filled.0[0][2] = Representation::Circle;
        third_column_filled.0[1][2] = Representation::Circle;
        third_column_filled.0[2][2] = Representation::Circle;

        assert_eq!(
            find_winner(&first_column_filled).unwrap(),
            Representation::Circle
        );
        assert_eq!(
            find_winner(&second_column_filled).unwrap(),
            Representation::Plus
        );
        assert_eq!(
            find_winner(&third_column_filled).unwrap(),
            Representation::Circle
        );
    }

    #[test]
    fn find_winner_left_to_right() {
        let mut board = Board::new();
        board.0[0][0] = Representation::Circle;
        board.0[1][1] = Representation::Circle;
        board.0[2][2] = Representation::Circle;

        assert_eq!(find_winner(&board).unwrap(), Representation::Circle);
    }

    #[test]
    fn find_winner_right_to_left() {
        let mut board = Board::new();
        board.0[0][2] = Representation::Plus;
        board.0[1][1] = Representation::Plus;
        board.0[2][0] = Representation::Plus;

        assert_eq!(find_winner(&board).unwrap(), Representation::Plus);
    }
}
