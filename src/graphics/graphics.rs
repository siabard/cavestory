use crate::graphics::Sprite;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use std::collections::HashMap;
use std::path::Path;

pub struct Graphics<'a> {
    pub sprite_sheets: HashMap<String, Texture<'a>>,
}

impl<'a> Graphics<'a> {
    pub fn new() -> Graphics<'a> {
        Graphics {
            sprite_sheets: HashMap::new(),
        }
    }

    pub fn load_image(
        &mut self,
        texture_creator: &'a mut TextureCreator<WindowContext>,
        image_name: String,
        path: &Path,
    ) -> &Texture<'a> {
        if let Ok(texture) = texture_creator.load_texture(&path) {
            self.sprite_sheets.insert(image_name.clone(), texture);
        }

        self.sprite_sheets.get(&image_name).unwrap()
    }

    pub fn render_sprite(&mut self, canvas: &mut WindowCanvas, sprite: &Sprite, x: i32, y: i32) {
        if let Some(texture) = self.sprite_sheets.get(&sprite.name) {
            let src = sprite.source_rect;
            let dest = Rect::new(
                x,
                y,
                sprite.source_rect.width(),
                sprite.source_rect.height(),
            );

            canvas
                .copy_ex(texture, Some(src), Some(dest), 0.0, None, false, false)
                .unwrap();
        }
    }
}
