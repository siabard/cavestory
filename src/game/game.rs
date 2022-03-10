use std::path::Path;

use sdl2::{
    rect::Rect,
    render::{TextureCreator, WindowCanvas},
    video::WindowContext,
};

use crate::graphics::{AnimateSprite, Graphics};

#[derive(Default)]
pub struct Game<'a> {
    pub player: Option<AnimateSprite>,
    pub graphics: Graphics<'a>,
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        Game { player: None, graphics: Graphics::new() }
    }

    pub fn init_sprite(&mut self, texture_creator: &'a mut TextureCreator<WindowContext>) {
        let mut player = AnimateSprite::new("player".into(), 0, 0, 150);
        player.add_animation("move_left".into(), Rect::new(0, 0, 16, 16), 3, 2);
        player.set_animation("move_left".into());
        self.player = Some(player);
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

    pub fn update(&mut self, dt: u32) {
        self.player.as_mut().unwrap().update(dt);
    }
}
