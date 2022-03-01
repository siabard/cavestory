use crate::game::MAX_FRAME_TIME;
use sdl2::{rect::Rect, render::WindowCanvas};

pub struct AnimateSprite {
    pub name: String,
    pub sprites: Vec<Rect>,
    pub frame_index: usize,
    pub time_elapsed: u32,
    pub visible: bool,
}

impl AnimateSprite {
    pub fn new(name: String, rect: Rect, frame: usize, visible: bool) -> AnimateSprite {
        // Rect에 대해서 frame수 만큼 새로운 Rect를 배열로 만든다.
        let mut sprites = vec![];

        for i in 0..frame {
            let r = Rect::new(
                rect.x + (rect.width() * i as u32) as i32,
                rect.y,
                rect.width(),
                rect.height(),
            );
            sprites.push(r);
        }

        AnimateSprite {
            name,
            sprites,
            frame_index: 0,
            time_elapsed: 0,
            visible,
        }
    }

    pub fn update(&mut self, dt: u32) {
        self.time_elapsed += dt;
        if self.time_elapsed > MAX_FRAME_TIME {
            self.time_elapsed = 0;

            self.frame_index += 1;
            if self.frame_index >= self.sprites.len() {
                self.frame_index = 0;
            }
        }
    }

    pub fn render(&self, canvas: &mut WindowCanvas) {}
}
