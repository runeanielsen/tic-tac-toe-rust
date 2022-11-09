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

pub struct Board(pub [[Symbol; 3]; 3]);

impl Board {
    pub fn new() -> Board {
        Board([[Symbol::Empty; 3]; 3])
    }

    pub fn place(&mut self, symbol: Symbol, player_move: [usize; 2]) {
        self.0[player_move[0]][player_move[1]] = symbol;
    }

    pub fn is_valid_move(&self, player_move: [usize; 2]) -> Result<bool, PlayerMoveError> {
        if player_move[0] > 2 || player_move[1] > 2 {
            return Err(PlayerMoveError::OutsideBoard(String::from(
                "The move is invalid because it is outside the board.",
            )));
        }

        if self.0[player_move[0]][player_move[1]] != Symbol::Empty {
            return Err(PlayerMoveError::FilledPosition(String::from(
                "The position is already filled.",
            )));
        }

        Ok(true)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board_representation = self
            .0
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
            assert!(board.is_valid_move(valid_move).unwrap());
        }
    }

    #[test]
    fn valid_player_move_symbols_on_board() {
        let mut board = Board::new();
        board.0[1][1] = Symbol::Plus;
        board.0[2][2] = Symbol::Circle;

        let valid_moves = [[0, 0], [0, 1], [0, 2], [1, 0], [1, 2], [2, 0], [2, 1]];

        for valid_move in valid_moves {
            assert!(board.is_valid_move(valid_move).unwrap());
        }
    }

    #[test]
    fn invalid_player_move_already_filled_slot() {
        let mut board = Board::new();
        board.0[1][1] = Symbol::Plus;

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
}
