use sdl2::image::LoadTexture;
use sdl2::render::WindowCanvas;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use std::collections::HashMap;
use std::path::Path;

use super::Renderable;

#[derive(Default)]
pub struct Graphics<'a> {
    pub sprite_sheets: HashMap<String, Texture<'a>>,
}

impl<'a> Graphics<'a> {
    pub fn new() -> Graphics<'a> {
        Graphics { sprite_sheets: HashMap::new() }
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

    pub fn render_sprite(&self, canvas: &mut WindowCanvas, sprite: &dyn Renderable) {
        if let Some(texture) = self.sprite_sheets.get(&sprite.get_name()) {
            sprite.render(canvas, texture);
        }
    }
}
