use sdl2::{
    rect::Rect,
    render::{Texture, WindowCanvas},
};

use crate::graphics::{AnimateSprite, Renderable};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    IdleLeft,
    IdleRight,
}

pub struct Player {
    animation: AnimateSprite,
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    facing: Direction,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        let mut animation = AnimateSprite::new("player".into(), 0, 0, 150);
        animation.add_animation("idle_left".into(), Rect::new(0, 0, 16, 16), 1, 1);
        animation.add_animation("idle_right".into(), Rect::new(0, 16, 16, 16), 1, 1);
        animation.add_animation("move_left".into(), Rect::new(0, 0, 16, 16), 3, 1);
        animation.add_animation("move_right".into(), Rect::new(0, 16, 16, 16), 3, 1);
        animation.set_animation("move_left".into());
        Self { animation, x, y, dx: 0, dy: 0, facing: Direction::IdleLeft }
    }

    pub fn update(&mut self, dt: u32) {
        self.animation.update(dt);
        self.x += ((self.dx * dt as i32) as f64 / 500.) as i32;
        self.y += ((self.dy * dt as i32) as f64 / 500.) as i32;

        self.facing = match self.dy {
            dy if dy > 0 => Direction::Down,
            dy if dy < 0 => Direction::Up,
            dy if dy == 0 => {
                if self.facing == Direction::Down {
                    Direction::IdleRight
                } else if self.facing == Direction::Up {
                    Direction::IdleLeft
                } else {
                    self.facing
                }
            }
            _ => self.facing,
        };

        self.facing = match self.dx {
            dx if dx > 0 => Direction::Right,
            dx if dx < 0 => Direction::Left,
            dx if dx == 0 => {
                if self.facing == Direction::Right {
                    Direction::IdleRight
                } else if self.facing == Direction::Left {
                    Direction::IdleLeft
                } else {
                    self.facing
                }
            }
            _ => self.facing,
        };

        match self.facing {
            Direction::IdleLeft => self.animation.set_animation("idle_left".into()),
            Direction::IdleRight => self.animation.set_animation("idle_right".into()),
            Direction::Left => self.animation.set_animation("move_left".into()),
            Direction::Right => self.animation.set_animation("move_right".into()),
            Direction::Up => self.animation.set_animation("move_left".into()),
            Direction::Down => self.animation.set_animation("move_right".into()),
        }
    }

    pub fn move_vector(&mut self, vector: (i32, i32)) {
        self.dx = 100 * vector.0;
        self.dy = 100 * vector.1;
    }
    pub fn stop(&mut self) {
        self.dx = 0;
        self.dy = 0;
    }
}

impl Renderable for Player {
    fn get_name(&self) -> String {
        "player".into()
    }

    fn render(&self, canvas: &mut WindowCanvas, texture: &Texture) {
        self.animation.render(self.x, self.y, canvas, texture);
    }
}
