#![warn(clippy::all, clippy::pedantic)]

mod board;
mod game;

use crate::game::start;

fn main() {
    start();
}
