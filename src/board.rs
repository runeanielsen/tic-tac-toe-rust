use std::convert::Into;
use std::fmt::Display;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Symbol {
    Empty,
    Plus,
    Circle,
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

#[derive(Clone)]
pub struct Board(pub [[Symbol; 3]; 3]);

impl Board {
    pub fn new() -> Board {
        Board([[Symbol::Empty; 3]; 3])
    }

    pub fn place(&mut self, symbol: Symbol, player_move: [usize; 2]) {
        self.0[player_move[0]][player_move[1]] = symbol;
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
