use std::convert::Into;
use std::fmt::Display;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Symbol {
    Empty,
    Plus,
    Circle,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PlayerMoveError {
    FilledPosition(String),
    OutsideBoard(String),
}

impl From<Symbol> for &str {
    fn from(val: Symbol) -> Self {
        match val {
            Symbol::Empty => "-",
            Symbol::Plus => "+",
            Symbol::Circle => "o",
        }
    }
}

pub struct Board {
    pub tiles: [[Symbol; 3]; 3],
}

impl Board {
    pub fn new() -> Board {
        Board {
            tiles: [[Symbol::Empty; 3]; 3],
        }
    }

    pub fn place(&mut self, symbol: Symbol, player_move: [usize; 2]) {
        self.tiles[player_move[0]][player_move[1]] = symbol;
    }

    pub fn is_valid_move(&self, player_move: [usize; 2]) -> Result<bool, PlayerMoveError> {
        if player_move[0] > 2 || player_move[1] > 2 {
            return Err(PlayerMoveError::OutsideBoard(String::from(
                "The move is invalid because it is outside the board.",
            )));
        }

        if self.tiles[player_move[0]][player_move[1]] != Symbol::Empty {
            return Err(PlayerMoveError::FilledPosition(String::from(
                "The position is already filled.",
            )));
        }

        Ok(true)
    }

    pub fn winner(&self) -> Option<Symbol> {
        let board_state = self.tiles;

        // Check for row and column winner.
        for i in 0..board_state.len() {
            // Row winner
            if board_state[i][0] == board_state[i][1] && board_state[i][0] == board_state[i][2] {
                // Empty cannot be a winner :)
                if board_state[i][0] != Symbol::Empty {
                    return Some(board_state[i][0]);
                }
            }

            // Colum winner
            if board_state[0][i] == board_state[1][i] && board_state[0][i] == board_state[2][i] {
                // Empty cannot be a winner :)
                if board_state[0][i] != Symbol::Empty {
                    return Some(board_state[0][i]);
                }
            }
        }

        // Left to right winner
        if board_state[0][0] != Symbol::Empty
            && board_state[0][0] == board_state[1][1]
            && board_state[0][0] == board_state[2][2]
        {
            return Some(board_state[0][0]);
        }

        // Right to left winner
        if board_state[0][2] != Symbol::Empty
            && board_state[0][2] == board_state[1][1]
            && board_state[0][2] == board_state[2][0]
        {
            return Some(board_state[0][2]);
        }

        None
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board_representation = self
            .tiles
            .map(|row| format!("| {} |", row.map(Into::<&str>::into).join(" | ")))
            .join("\n");

        write!(f, "{}", board_representation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_able_to_place_a_symbol_on_the_board() {
        let mut board = Board::new();
        board.place(Symbol::Plus, [1, 1]);

        let mut expected = [[Symbol::Empty; 3]; 3];
        expected[1][1] = Symbol::Plus;

        (0..board.tiles.len()).for_each(|i| {
            for j in 0..board.tiles[i].len() {
                assert_eq!(board.tiles[i][j], expected[i][j]);
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
            assert!(board.is_valid_move(valid_move).unwrap());
        }
    }

    #[test]
    fn valid_player_move_symbols_on_board() {
        let mut board = Board::new();
        board.tiles[1][1] = Symbol::Plus;
        board.tiles[2][2] = Symbol::Circle;

        let valid_moves = [[0, 0], [0, 1], [0, 2], [1, 0], [1, 2], [2, 0], [2, 1]];

        for valid_move in valid_moves {
            assert!(board.is_valid_move(valid_move).unwrap());
        }
    }

    #[test]
    fn invalid_player_move_already_filled_slot() {
        let mut board = Board::new();
        board.tiles[1][1] = Symbol::Plus;

        assert_eq!(
            board.is_valid_move([1, 1]),
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
                board.is_valid_move(invalid_move),
                Err(PlayerMoveError::OutsideBoard(
                    "The move is invalid because it is outside the board.".to_string()
                ))
            );
        }
    }

    #[test]
    fn find_winner_row_winner_test() {
        let mut first_row_filled = Board::new();
        first_row_filled.tiles[0][0] = Symbol::Circle;
        first_row_filled.tiles[0][1] = Symbol::Circle;
        first_row_filled.tiles[0][2] = Symbol::Circle;

        let mut second_row_filled = Board::new();
        second_row_filled.tiles[1][0] = Symbol::Plus;
        second_row_filled.tiles[1][1] = Symbol::Plus;
        second_row_filled.tiles[1][2] = Symbol::Plus;

        let mut third_row_filled = Board::new();
        third_row_filled.tiles[2][0] = Symbol::Plus;
        third_row_filled.tiles[2][1] = Symbol::Plus;
        third_row_filled.tiles[2][2] = Symbol::Plus;

        assert_eq!(first_row_filled.winner().unwrap(), Symbol::Circle);
        assert_eq!(second_row_filled.winner().unwrap(), Symbol::Plus);
        assert_eq!(third_row_filled.winner().unwrap(), Symbol::Plus);
    }

    #[test]
    fn find_column_winner() {
        let mut first_column_filled = Board::new();
        first_column_filled.tiles[0][0] = Symbol::Circle;
        first_column_filled.tiles[1][0] = Symbol::Circle;
        first_column_filled.tiles[2][0] = Symbol::Circle;

        let mut second_column_filled = Board::new();
        second_column_filled.tiles[0][1] = Symbol::Plus;
        second_column_filled.tiles[1][1] = Symbol::Plus;
        second_column_filled.tiles[2][1] = Symbol::Plus;

        let mut third_column_filled = Board::new();
        third_column_filled.tiles[0][2] = Symbol::Circle;
        third_column_filled.tiles[1][2] = Symbol::Circle;
        third_column_filled.tiles[2][2] = Symbol::Circle;

        assert_eq!(first_column_filled.winner().unwrap(), Symbol::Circle);
        assert_eq!(second_column_filled.winner().unwrap(), Symbol::Plus);
        assert_eq!(third_column_filled.winner().unwrap(), Symbol::Circle);
    }

    #[test]
    fn find_winner_left_to_right() {
        let mut board = Board::new();
        board.tiles[0][0] = Symbol::Circle;
        board.tiles[1][1] = Symbol::Circle;
        board.tiles[2][2] = Symbol::Circle;

        assert_eq!(board.winner().unwrap(), Symbol::Circle);
    }

    #[test]
    fn find_winner_right_to_left() {
        let mut board = Board::new();
        board.tiles[0][2] = Symbol::Plus;
        board.tiles[1][1] = Symbol::Plus;
        board.tiles[2][0] = Symbol::Plus;

        assert_eq!(board.winner().unwrap(), Symbol::Plus);
    }

    #[test]
    fn can_convert_from_board_symbol_to_string() {
        let assertions = [
            (Symbol::Empty, "-"),
            (Symbol::Plus, "+"),
            (Symbol::Circle, "o"),
        ];

        for (value, expected) in assertions {
            assert_eq!(expected, Into::<&str>::into(value));
        }
    }
}
