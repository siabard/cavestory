use std::{collections::HashMap, path::Path};

use sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

pub struct TextureManager<'a> {
    pub textures: HashMap<&'static str, Texture<'a>>,
}

impl<'a> TextureManager<'a> {
    pub fn new() -> TextureManager<'a> {
        TextureManager { textures: HashMap::new() }
    }

    pub fn insert(
        &mut self,
        name: &'static str,
        texture_creator: &'a mut TextureCreator<WindowContext>,
        path: &Path,
    ) {
        let texture = texture_creator.load_texture(path).unwrap();
        self.textures.insert(name, texture);
    }

    pub fn get_texture(&self, name: &'static str) -> &Texture<'a> {
        self.textures.get(name).unwrap()
    }
}
