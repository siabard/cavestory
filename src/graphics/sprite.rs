use sdl2::{
    rect::Rect,
    render::{Texture, WindowCanvas},
};

use crate::game::SPRITE_SCALE;

#[derive(Debug, Clone)]
pub struct Sprite {
    pub name: String,
    pub source_rect: Rect,
}

impl Sprite {
    pub fn new(name: String, src_x: i32, src_y: i32, width: u32, height: u32) -> Sprite {
        let source_rect = Rect::new(src_x, src_y, width, height);
        Sprite { name, source_rect }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn render(&self, x: i32, y: i32, canvas: &mut WindowCanvas, texture: &Texture) {
        let dest = Rect::new(
            (x as f32 * SPRITE_SCALE) as i32,
            (y as f32 * SPRITE_SCALE) as i32,
            (self.source_rect.width() as f32 * SPRITE_SCALE) as u32,
            (self.source_rect.height() as f32 * SPRITE_SCALE) as u32,
        );

        canvas
            .copy_ex(texture, Some(self.source_rect), Some(dest), 0.0, None, false, false)
            .unwrap();
    }
}
