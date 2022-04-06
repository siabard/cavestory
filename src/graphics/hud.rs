use crate::{game::SPRITE_SCALE, player::Player};

use super::{Renderable, Sprite, Vector2};
use sdl2::render::{Texture, WindowCanvas};

pub struct Hud {
    health_bar: Sprite,
    health_digit: Vec<Sprite>,
    health_bar_pos: Vector2,
    health_digit_pos: Vector2,
    health: i32,
}

impl Hud {
    pub fn new() -> Self {
        let health_bar = Sprite::new("textbox".into(), 0, 40, 64, 8);
        let health_bar_pos = Vector2(35., 70.);
        let mut health_digit = vec![];
        for i in 0..9 {
            health_digit.push(Sprite::new("textbox".into(), 8 * i, 56, 8, 8));
        }

        let health_digit_pos = Vector2(35., 200.);

        Hud { health_bar, health_digit, health_bar_pos, health_digit_pos, health: 0 }
    }

    pub fn update(&mut self, player: &Player) {
        self.health = player.current_health;
    }
}

impl Renderable for Hud {
    fn get_name(&self) -> String {
        "textbox".into()
    }

    fn render(&self, canvas: &mut WindowCanvas, texture: &Texture) {
        self.health_bar.render(
            self.health_bar_pos.0 as i32,
            self.health_bar_pos.1 as i32,
            canvas,
            texture,
        );

        for i in 0..self.health.to_string().len() {
            let digit = self.health / 10_i32.pow(i as u32) % 10;
            self.health_digit.get(digit as usize).unwrap().render(
                (self.health_digit_pos.0 + ((i as f32) * 8.)) as i32,
                self.health_digit_pos.1 as i32,
                canvas,
                texture,
            );
        }
    }
}
