use crate::game::{MAX_FRAME_TIME, SPRITE_SCALE};
use sdl2::{rect::Rect, render::Texture, render::WindowCanvas};
use std::collections::HashMap;

use super::Renderable;

#[derive(Default)]
pub struct Animation {
    pub frames: Vec<Rect>,
}

impl Animation {
    pub fn new(rect: Rect, horizontal: usize, vertical: usize) -> Self {
        let mut frames = vec![];

        for y in 0..vertical {
            for x in 0..horizontal {
                let r = Rect::new(
                    rect.x + (rect.width() * x as u32) as i32,
                    rect.y + (rect.height() * y as u32) as i32,
                    rect.width(),
                    rect.height(),
                );
                frames.push(r);
            }
        }

        Self { frames }
    }
}

#[derive(Default)]
pub struct AnimateSprite {
    pub name: String,
    pub sprites: HashMap<String, Animation>,
    pub current_animation: String,
    pub frame_index: usize,
    pub time_elapsed: u32,
    pub frame_length: u32,
    pub visible: bool,
}

impl AnimateSprite {
    pub fn new(name: String, frame_index: usize, time_elapsed: u32, frame_length: u32) -> Self {
        Self {
            name,
            sprites: HashMap::new(),
            current_animation: "".into(),
            frame_index,
            time_elapsed,
            frame_length,
            visible: false,
        }
    }

    pub fn add_animation(&mut self, name: String, rect: Rect, horizontal: usize, vertical: usize) {
        let animation = Animation::new(rect, horizontal, vertical);
        self.sprites.insert(name, animation);
    }

    pub fn set_animation(&mut self, name: String) {
        if self.current_animation.ne(&name) {
            self.current_animation = name;
            self.frame_index = 0;
            self.time_elapsed = 0;
        }
    }

    pub fn update(&mut self, dt: u32) {
        let current_animation = self.sprites.get(&self.current_animation).unwrap();

        self.time_elapsed += dt;
        if self.time_elapsed > self.frame_length {
            self.time_elapsed = 0;

            self.frame_index += 1;
        }

        if self.frame_index >= current_animation.frames.len() {
            self.frame_index = 0;
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn render(&self, x: i32, y: i32, canvas: &mut WindowCanvas, texture: &Texture) {
        let current_animation = self.sprites.get(&(self.current_animation)).unwrap();

        let current_sprite = current_animation.frames[self.frame_index];

        let dest = Rect::new(
            x,
            y,
            (current_sprite.width() as f32 * SPRITE_SCALE) as u32,
            (current_sprite.height() as f32 * SPRITE_SCALE) as u32,
        );

        canvas.copy_ex(texture, Some(current_sprite), Some(dest), 0.0, None, false, false).unwrap();
    }
}
