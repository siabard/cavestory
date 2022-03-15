use std::path::Path;

use sdl2::{
    render::{TextureCreator, WindowCanvas},
    video::WindowContext,
};

use crate::graphics::{Graphics, Sprite};

#[derive(Default)]
pub struct Game<'a> {
    pub player: Option<Sprite>,
    pub graphics: Graphics<'a>,
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        Game { player: None, graphics: Graphics::new() }
    }

    pub fn init_sprite(&mut self, texture_creator: &'a TextureCreator<WindowContext>) {
        self.player = Some(Sprite::new("player".into(), 0, 0, 16, 16, 100, 100));
        self.graphics.load_image(
            texture_creator,
            "player".into(),
            Path::new("resources/mychar.png"),
        );
    }

    pub fn render(&self, canvas: &mut WindowCanvas) {
        let player = self.player.as_ref().unwrap();
        self.graphics.render_sprite(canvas, player);
    }

    pub fn update(&mut self, _dt: u32) {
        // do nothing
    }
}
