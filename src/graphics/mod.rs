pub mod animate_sprite;
pub mod graphics;
pub mod level;
pub mod sprite;
pub mod texture_manager;
pub mod tile;

pub use animate_sprite::*;
pub use graphics::*;
use sdl2::render::{Texture, WindowCanvas};
pub use sprite::*;
pub use texture_manager::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2(f32, f32);

impl From<(f32, f32)> for Vector2 {
    fn from(point: (f32, f32)) -> Self {
        Vector2(point.0, point.1)
    }
}

pub trait Renderable {
    fn get_name(&self) -> String;
    fn render(&self, canvas: &mut WindowCanvas, texture: &Texture);
}
