use crate::game::SPRITE_SCALE;
use crate::graphics::animation::Animation;
use sdl2::{rect::Rect, render::Texture, render::WindowCanvas};
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct AnimateSprite {
    pub name: String,
    pub sprites: HashMap<String, Animation>,
    pub current_animation: String,
}

impl AnimateSprite {
    pub fn new(name: String) -> Self {
        Self { name, sprites: HashMap::new(), current_animation: "".into() }
    }

    pub fn add_animation(
        &mut self,
        name: String,
        rect: Rect,
        frame_duration: u32,
        play_once: bool,
        horizontal: usize,
        vertical: usize,
    ) {
        let animation = Animation::new(rect, frame_duration, play_once, horizontal, vertical);
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

        if current_animation.visible {
            let current_sprite = current_animation.get_current_frame();

            let dest = Rect::new(
                (x as f32 * SPRITE_SCALE) as i32,
                (y as f32 * SPRITE_SCALE) as i32,
                (current_sprite.width() as f32 * SPRITE_SCALE) as u32,
                (current_sprite.height() as f32 * SPRITE_SCALE) as u32,
            );

            canvas
                .copy_ex(texture, Some(current_sprite), Some(dest), 0.0, None, false, false)
                .unwrap();
        }
    }
}
