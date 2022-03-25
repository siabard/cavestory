pub mod animate_sprite;
pub mod graphics;
pub mod level;
pub mod sprite;
pub mod texture_manager;
pub mod tile;

pub use animate_sprite::*;
pub use graphics::*;
use sdl2::{
    rect::Rect,
    render::{Texture, WindowCanvas},
};
pub use sprite::*;
pub use texture_manager::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2(f32, f32);

impl From<(f32, f32)> for Vector2 {
    fn from(point: (f32, f32)) -> Self {
        Vector2(point.0, point.1)
    }
}

impl Vector2 {
    pub fn left(&self) -> f32 {
        self.0
    }

    pub fn top(&self) -> f32 {
        self.1
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl From<Rect> for Rectangle {
    fn from(rect: Rect) -> Self {
        Rectangle {
            left: rect.x as f32,
            right: (rect.x + rect.w) as f32,
            top: rect.y as f32,
            bottom: (rect.y + rect.h) as f32,
        }
    }
}

pub trait Renderable {
    fn get_name(&self) -> String;
    fn render(&self, canvas: &mut WindowCanvas, texture: &Texture);
}
