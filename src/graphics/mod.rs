pub mod animate_sprite;
pub mod graphics;
pub mod sprite;
pub mod texture_manager;

pub use animate_sprite::*;
pub use graphics::*;
use sdl2::render::{Texture, WindowCanvas};
pub use sprite::*;
pub use texture_manager::*;

pub trait Renderable {
    fn get_name(&self) -> String;
    fn render(&self, canvas: &mut WindowCanvas, texture: &Texture);
}
