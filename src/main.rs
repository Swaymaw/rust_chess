mod game;
mod utils;

use game::*;

fn main() {
    let game = Game::initialize();
    println!("{}", game.to_string());
}