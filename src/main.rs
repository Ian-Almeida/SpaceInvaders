mod game;
mod entities;
extern crate piston_window;

use game::{Game};

fn main() {

    let mut game = Game::default();

    game.start_game();
}