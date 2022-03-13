use std::path::Path;

use sdl2::{
    render::{TextureCreator, WindowCanvas},
    video::WindowContext,
};

use crate::{graphics::Graphics, input::Input, player::Player};

fn bool_to_sign(b: bool) -> i32 {
    if b {
        1
    } else {
        0
    }
}

#[derive(Default)]
pub struct Game<'a> {
    pub player: Option<Player>,
    pub graphics: Graphics<'a>,
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        Game { player: None, graphics: Graphics::new() }
    }

    pub fn init_sprite(&mut self, texture_creator: &'a mut TextureCreator<WindowContext>) {
        let player = Player::new(100, 100);
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

    pub fn process_key_event(&mut self, input: &Input) {
        let player = self.player.as_mut().unwrap();

        player.move_vector((
            bool_to_sign(input.is_key_held(sdl2::keyboard::Scancode::Right))
                - bool_to_sign(input.is_key_held(sdl2::keyboard::Scancode::Left)),
            bool_to_sign(input.is_key_held(sdl2::keyboard::Scancode::Down))
                - bool_to_sign(input.is_key_held(sdl2::keyboard::Scancode::Up)),
        ));
    }
}
