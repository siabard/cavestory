use std::path::Path;

use sdl2::{
    render::{TextureCreator, WindowCanvas},
    video::WindowContext,
};

use crate::graphics::{Graphics, Sprite};

const FPS: u32 = 50;
const MAX_FRAME_TIME: u32 = 5 * 1000 / FPS;

pub struct Game<'a> {
    pub player: Option<Sprite>,
    pub graphics: Graphics<'a>,
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        Game {
            player: None,
            graphics: Graphics::new(),
        }
    }

    pub fn init_sprite(&mut self, texture_creator: &'a mut TextureCreator<WindowContext>) {
        self.graphics.load_image(
            texture_creator,
            "player".to_owned(),
            Path::new("resources/mychar.png"),
        );

        self.player = Some(Sprite::new("player".to_owned(), 0, 0, 16, 16, 100.0, 100.0));
    }

    pub fn render(&mut self, canvas: &mut WindowCanvas) {
        self.graphics
            .render_sprite(canvas, self.player.as_ref().unwrap(), 100, 100);
    }
}
