mod cell;
mod game;
mod terminal;

use game::Game;

fn main() {
    let mut game = Game::new(8);
    game.run();
}
