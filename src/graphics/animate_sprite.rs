use crate::game::SPRITE_SCALE;
use sdl2::{rect::Rect, render::Texture, render::WindowCanvas};
use std::collections::HashMap;

#[derive(Default)]
pub struct Animation {
    frames: Vec<Rect>,
    frame_index: usize,
    time_elapsed: u32,
    frame_duration: u32,
}

impl Animation {
    pub fn new(rect: Rect, frame_length: u32, horizontal: usize, vertical: usize) -> Self {
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

        Self { frames, frame_index: 0, time_elapsed: 0, frame_duration: frame_length }
    }

    pub fn get_current_frame(&self) -> Rect {
        self.frames[self.frame_index]
    }

    pub fn update(&mut self, dt: u32) {
        self.time_elapsed += dt;
        if self.time_elapsed > self.frame_duration {
            self.time_elapsed = 0;

            self.frame_index += 1;
        }

        if self.frame_index >= self.frames.len() {
            self.frame_index = 0;
        }
    }
}

#[derive(Default)]
pub struct AnimateSprite {
    pub name: String,
    pub sprites: HashMap<String, Animation>,
    pub current_animation: String,
    pub visible: bool,
}

impl AnimateSprite {
    pub fn new(name: String) -> Self {
        Self { name, sprites: HashMap::new(), current_animation: "".into(), visible: false }
    }

    pub fn add_animation(
        &mut self,
        name: String,
        rect: Rect,
        frame_duration: u32,
        horizontal: usize,
        vertical: usize,
    ) {
        let animation = Animation::new(rect, frame_duration, horizontal, vertical);
        self.sprites.insert(name, animation);
    }

    pub fn set_animation(&mut self, name: String) {
        if self.current_animation.ne(&name) {
            self.current_animation = name;
        }
    }

    pub fn update(&mut self, dt: u32) {
        let current_animation = self.sprites.get_mut(&self.current_animation).unwrap();
        current_animation.update(dt);
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn render(&self, x: i32, y: i32, canvas: &mut WindowCanvas, texture: &Texture) {
        let current_animation = self.sprites.get(&(self.current_animation)).unwrap();

        let current_sprite = current_animation.get_current_frame();

        let dest = Rect::new(
            (x as f32 * SPRITE_SCALE) as i32,
            (y as f32 * SPRITE_SCALE) as i32,
            (current_sprite.width() as f32 * SPRITE_SCALE) as u32,
            (current_sprite.height() as f32 * SPRITE_SCALE) as u32,
        );

        canvas.copy_ex(texture, Some(current_sprite), Some(dest), 0.0, None, false, false).unwrap();
    }
}
