use std::path::Path;

use sdl2::{
    rect::Rect,
    render::{TextureCreator, WindowCanvas},
    video::WindowContext,
};

use crate::{
    graphics::{map::Map, Graphics},
    input::Input,
    player::Player,
};

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
    pub map: Option<Map<'a>>,
    pub graphics: Graphics<'a>,
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        Game { player: None, map: None, graphics: Graphics::new() }
    }

    pub fn init_sprite(&mut self, texture_creator: &'a TextureCreator<WindowContext>) {
        let player = Player::new(100, 100);
        self.player = Some(player);
        self.graphics.load_image(
            texture_creator,
            "player".into(),
            Path::new("resources/mychar.png"),
        );

        let map = Map::new("map".into(), texture_creator, "tiled_base64_zlib.tmx");
        self.map = Some(map);
    }

    pub fn render(&self, canvas: &mut WindowCanvas) {
        if let Some(map) = &self.map {
            map.render(canvas, &Rect::new(0, 0, 320, 240));
        }

        if let Some(player) = &self.player {
            self.graphics.render_sprite(canvas, player);
        }
    }

    pub fn update(&mut self, dt: u32) {
        if let Some(player) = self.player.as_mut() {
            player.update(dt);
        }
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
