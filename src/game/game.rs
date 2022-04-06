use std::path::Path;

use sdl2::{
    rect::Rect,
    render::{TextureCreator, WindowCanvas},
    video::WindowContext,
};

use super::{SCREEN_HEIGHT, SCREEN_WIDTH, SPRITE_SCALE};
use crate::{
    graphics::{level::Level, Graphics, Hud},
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
    pub hud: Option<Hud>,
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        Game { player: None, level: HashMap::new(), graphics: Graphics::new(), hud: None }
    }

    pub fn init_sprite(&mut self, texture_creator: &'a TextureCreator<WindowContext>) {
        let player = Player::new(100, 100);
        self.player = Some(player);

        let hud = Hud::new();
        self.hud = Some(hud);

        self.graphics.load_image(
            texture_creator,
            "player".into(),
            Path::new("resources/mychar.png"),
        );
        self.graphics.load_image(
            texture_creator,
            "textbox".into(),
            Path::new("resources/text_box.png"),
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
            if let Some(hud) = &self.hud {
                self.graphics.render_sprite(canvas, hud);
            }
        }
    }

    pub fn update(&mut self, dt: u32) {
        if let Some(player) = self.player.as_mut() {
            player.update(dt);
            if let Some(hud) = self.hud.as_mut() {
                hud.update(player);
            }

            if let Some(level) = self.level.get_mut("map") {
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

                level.update(dt);
            }
        }
    }

    pub fn process_key_event(&mut self, input: &Input) {
        let player = self.player.as_mut().unwrap();

        if input.is_key_held(sdl2::keyboard::Scancode::Right) {
            player.move_right();
        } else if input.is_key_held(sdl2::keyboard::Scancode::Left) {
            player.move_left();
        } else if !input.is_key_held(sdl2::keyboard::Scancode::Right)
            && !input.is_key_held(sdl2::keyboard::Scancode::Left)
        {
            player.stop();
        }

        if input.is_key_held(sdl2::keyboard::Scancode::Up) {
            player.look_up();
        } else if input.is_key_held(sdl2::keyboard::Scancode::Down) {
            player.look_down();
        }

        if input.was_key_release(sdl2::keyboard::Scancode::Up) {
            player.stop_looking_up();
        }
        if input.was_key_release(sdl2::keyboard::Scancode::Down) {
            player.stop_looking_down();
        }

        if input.is_key_held(sdl2::keyboard::Scancode::Z) {
            player.jump();
        }
    }
}
