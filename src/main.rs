use cavestory::game::Game;

fn main() {
    // need a window on the screen to draw our game
    let mut game = Game::new();

    game.game_loop();
}
