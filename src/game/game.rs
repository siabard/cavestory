use std::path::Path;

use sdl2::{
    rect::Rect,
    render::{TextureCreator, WindowCanvas},
    video::WindowContext,
};

use super::{SCREEN_HEIGHT, SCREEN_WIDTH, SPRITE_SCALE};
use crate::{
    graphics::{level::Level, Graphics},
    input::Input,
    player::Player,
};
use std::collections::HashMap;

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
    pub level: HashMap<String, Level<'a>>,
    pub graphics: Graphics<'a>,
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        Game { player: None, level: HashMap::new(), graphics: Graphics::new() }
    }

    pub fn init_sprite(&mut self, texture_creator: &'a TextureCreator<WindowContext>) {
        let player = Player::new(100, 100);
        self.player = Some(player);
        self.graphics.load_image(
            texture_creator,
            "player".into(),
            Path::new("resources/mychar.png"),
        );

        let map = Level::new(texture_creator, "stage.tmx");
        self.level.insert("map".into(), map);
    }

    pub fn render(&self, canvas: &mut WindowCanvas) {
        if let Some(map) = self.level.get("map") {
            map.render(
                canvas,
                &Rect::new(
                    0,
                    0,
                    (SCREEN_WIDTH as f32 / SPRITE_SCALE) as u32,
                    (SCREEN_HEIGHT as f32 / SPRITE_SCALE) as u32,
                ),
            );
        }

        if let Some(player) = &self.player {
            self.graphics.render_sprite(canvas, player);
        }
    }

    pub fn update(&mut self, dt: u32) {
        if let Some(player) = self.player.as_mut() {
            player.update(dt);

            if let Some(level) = self.level.get("map") {
                // collision
                let collided_blocks = level.collided_blocks(&player.collision);
                if !collided_blocks.is_empty() {
                    player.handle_tile_collision(&collided_blocks);
                }

                // collision slope
                let collided_slopes = level.collided_slopes(&player.collision);
                if !collided_slopes.is_empty() {
                    player.handle_slope_collision(&collided_slopes);
                }
            }
        }
    }

    pub fn process_key_event(&mut self, input: &Input) {
        let player = self.player.as_mut().unwrap();

        player.move_vector((
            bool_to_sign(input.is_key_held(sdl2::keyboard::Scancode::Right))
                - bool_to_sign(input.is_key_held(sdl2::keyboard::Scancode::Left)),
            0,
        ));

        if input.is_key_held(sdl2::keyboard::Scancode::Up) {
            player.jump();
        }
    }
}
