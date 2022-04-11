use crate::player::Player;

use super::{Renderable, Sprite, Vector2};
use sdl2::render::{Texture, WindowCanvas};

pub struct Hud {
    health_bar: Sprite,
    health_digit: Vec<Sprite>,
    health_bar_pos: Vector2,
    health_digit_pos: Vector2,
    health: i32,
    current_health_bar: Sprite,
    current_health_pos: Vector2,
    lvl_word: Sprite,
    lvl_word_pos: Vector2,
    lvl_number: Sprite,
    lvl_number_pos: Vector2,
    exp_bar: Sprite,
    exp_bar_pos: Vector2,
    slash: Sprite,
    slash_pos: Vector2,
    dashes: Sprite,
    dashes_pos: Vector2,
}

impl Hud {
    pub fn new() -> Self {
        let health_bar = Sprite::new("textbox".into(), 0, 40, 64, 8);
        let health_bar_pos = Vector2(16., 35.);
        let mut health_digit = vec![];
        for i in 0..9 {
            health_digit.push(Sprite::new("textbox".into(), 8 * i, 56, 8, 8));
        }
        let health_digit_pos = Vector2(32., 35.);

        let current_health_bar = Sprite::new("textbox".into(), 0, 25, 39, 5);
        let current_health_pos = Vector2(40., 36.);

        let lvl_word = Sprite::new("textbox".into(), 81, 81, 11, 7);
        let lvl_number = Sprite::new("textbox".into(), 0, 56, 8, 1);
        let exp_bar = Sprite::new("textbox".into(), 0, 72, 40, 8);
        let slash = Sprite::new("textbox".into(), 72, 48, 8, 8);
        let dashes = Sprite::new("textbox".into(), 81, 51, 15, 11);

        let lvl_word_pos = Vector2(16., 26.);
        let lvl_number_pos: Vector2 = Vector2(32., 26.);
        let exp_bar_pos = Vector2(40., 26.);
        let slash_pos = Vector2(50., 17.);
        let dashes_pos = Vector2(66., 15.);

        Hud {
            health_bar,
            health_digit,
            health_bar_pos,
            health_digit_pos,
            health: 0,
            current_health_bar,
            current_health_pos,

            lvl_word,
            lvl_word_pos,
            lvl_number,
            lvl_number_pos,
            exp_bar,
            exp_bar_pos,
            slash,
            slash_pos,
            dashes,
            dashes_pos,
        }
    }

    pub fn update(&mut self, player: &Player) {
        let max_health = player.max_health;
        self.health = player.current_health;

        let ratio: f32 = self.health as f32 / max_health as f32;
        self.current_health_bar.source_rect.set_width((ratio * 39.) as u32);
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

        self.current_health_bar.render(
            self.current_health_pos.0 as i32,
            self.current_health_pos.1 as i32,
            canvas,
            texture,
        );

        self.lvl_number.render(
            self.lvl_number_pos.0 as i32,
            self.lvl_number_pos.1 as i32,
            canvas,
            texture,
        );

        self.lvl_word.render(
            self.lvl_word_pos.0 as i32,
            self.lvl_word_pos.1 as i32,
            canvas,
            texture,
        );

        self.exp_bar.render(self.exp_bar_pos.0 as i32, self.exp_bar_pos.1 as i32, canvas, texture);

        self.slash.render(self.slash_pos.0 as i32, self.slash_pos.1 as i32, canvas, texture);

        self.dashes.render(self.dashes_pos.0 as i32, self.dashes_pos.1 as i32, canvas, texture);
    }
}
