pub mod constant;
pub mod game;
pub mod graphics;
pub mod input;
pub mod physics;
pub mod player;

pub enum GameResult {
    None,
    GotoMap(String),
}
