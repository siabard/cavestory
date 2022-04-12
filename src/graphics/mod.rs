pub mod animate_sprite;
pub mod animated_tile;
pub mod animation;
pub mod door;
pub mod graphics;
pub mod hud;
pub mod level;
pub mod sprite;
pub mod texture_manager;
pub mod tile;

pub use animate_sprite::*;
pub use animated_tile::*;
pub use animation::*;
pub use door::*;
pub use graphics::*;
pub use hud::*;

use sdl2::{
    rect::Rect,
    render::{Texture, WindowCanvas},
};
pub use sprite::*;
pub use texture_manager::*;

use crate::physics::{Sided, Sides};

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
    pub height: f32,
    pub width: f32,
}

impl From<Rect> for Rectangle {
    fn from(rect: Rect) -> Self {
        Rectangle {
            left: rect.x as f32,
            right: (rect.x + rect.w) as f32,
            top: rect.y as f32,
            bottom: (rect.y + rect.h) as f32,
            width: rect.w as f32,
            height: rect.h as f32,
        }
    }
}

impl From<Rectangle> for Rect {
    fn from(rectangle: Rectangle) -> Self {
        Rect::new(
            rectangle.left as i32,
            rectangle.top as i32,
            rectangle.width as u32,
            rectangle.height as u32,
        )
    }
}

impl Sided for Rectangle {
    fn get_rect(&self) -> Rect {
        (*self).into()
    }

    fn get_side(&self, side: Sides) -> i32 {
        match side {
            Sides::Left => self.left as i32,
            Sides::Right => (self.left + self.width) as i32,
            Sides::Top => self.top as i32,
            Sides::Bottom => (self.top + self.height) as i32,
            Sides::None => 0,
        }
    }
}

impl Rectangle {
    pub fn center_x(&self) -> f32 {
        (self.left + self.right) / 2.
    }

    pub fn center_y(&self) -> f32 {
        (self.top + self.bottom) / 2.
    }
}

pub trait Renderable {
    fn get_name(&self) -> String;
    fn render(&self, canvas: &mut WindowCanvas, texture: &Texture);
}
