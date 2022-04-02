use sdl2::{
    rect::Rect,
    render::{Texture, WindowCanvas},
};
use tiled::Frame;

#[derive(Debug, Clone)]
pub struct AnimatedTile {
    position: Rect,
    animation: Vec<Frame>,
    index: usize,
    duration: i32,
}

impl AnimatedTile {
    pub fn new(position: Rect, animation: Vec<Frame>, index: usize, duration: i32) -> Self {
        AnimatedTile { position, animation, index, duration }
    }

    pub fn update(&mut self, dt: u32) {
        if self.duration <= 0 {
            if self.index == self.animation.len() - 1 {
                self.index = 0;
            } else {
                self.index += 1;
                self.duration = self.animation[self.index].duration as i32;
            }
        } else {
            self.duration -= dt as i32;
        }
    }

    pub fn get_current_frame(&self) -> u32 {
        self.animation[self.index].tile_id
    }
}
