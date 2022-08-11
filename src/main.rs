pub mod game;

use crate::game::Game;

fn main() {
    println!("Welcome to Chess 1.0");
    let mut game = Game::new();
    game.start_game_loop();
}
