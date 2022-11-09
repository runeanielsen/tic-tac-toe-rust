use std::convert::Into;
use std::fmt::Display;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Representation {
    Empty,
    Plus,
    Circle,
}

impl From<Representation> for &str {
    fn from(val: Representation) -> Self {
        match val {
            Representation::Empty => "-",
            Representation::Plus => "+",
            Representation::Circle => "o",
        }
    }
}

#[derive(Clone)]
pub struct Board(pub [[Representation; 3]; 3]);

impl Board {
    pub fn new() -> Board {
        Board([[Representation::Empty; 3]; 3])
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
