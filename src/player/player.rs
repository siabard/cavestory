use sdl2::{
    rect::Rect,
    render::{Texture, WindowCanvas},
};

use crate::{
    graphics::{AnimateSprite, Renderable},
    physics::Sides,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    IdleLeft,
    IdleRight,
    Idle,
}

impl From<Sides> for Direction {
    fn from(side: Sides) -> Direction {
        match side {
            Sides::Bottom => Direction::Down,
            Sides::Top => Direction::Up,
            Sides::Left => Direction::Left,
            Sides::Right => Direction::Right,
            Sides::None => Direction::Idle,
        }
    }
}

pub const GRAVITY: f32 = 0.02;
pub const GRAVITY_CAP: f32 = 0.8;
pub const WALK_SPEED: f32 = 0.2;

pub struct Player {
    animation: AnimateSprite,
    x: i32,
    y: i32,
    dx: f32,
    dy: f32,
    facing: Direction,
    grounded: bool,
    pub collision: Rect,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        let mut animation = AnimateSprite::new("player".into(), 0, 0, 150);
        animation.add_animation("idle_left".into(), Rect::new(0, 0, 16, 16), 1, 1);
        animation.add_animation("idle_right".into(), Rect::new(0, 16, 16, 16), 1, 1);
        animation.add_animation("move_left".into(), Rect::new(0, 0, 16, 16), 3, 1);
        animation.add_animation("move_right".into(), Rect::new(0, 16, 16, 16), 3, 1);
        animation.set_animation("move_left".into());
        Self {
            animation,
            x,
            y,
            dx: 0.0,
            dy: 0.0,
            facing: Direction::IdleLeft,
            grounded: false,
            collision: Rect::new(0, 0, 16, 16),
        }
    }

    pub fn update(&mut self, dt: u32) {
        self.animation.update(dt);
        self.x += (self.dx * dt as f32) as i32;

        // free fall
        if self.dy <= GRAVITY_CAP {
            self.dy += GRAVITY;
        }
        self.y += (self.dy * dt as f32) as i32;

        self.facing = match self.dy {
            dy if dy > 0. => Direction::Down,
            dy if dy < 0. => Direction::Up,
            dy if dy == 0. => {
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
            dx if dx > 0. => Direction::Right,
            dx if dx < 0. => Direction::Left,
            dx if dx == 0. => {
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
            _ => self.animation.set_animation("idle_right".into()),
        }

        self.collision.x = self.x;
        self.collision.y = self.y;
    }

    pub fn move_vector(&mut self, vector: (i32, i32)) {
        self.dx = WALK_SPEED * vector.0 as f32;
        // self.dy = WALK_SPEED * vector.1 as f32;
    }
    pub fn stop(&mut self) {
        self.dx = 0.0;
        // self.dy = 0.0;
    }

    pub fn handle_collision(&mut self, others: &[Rect]) {
        for other in others {
            let side = self.collision_side(other);
            match side {
                Sides::Top => {
                    self.y = other.y + other.height() as i32;
                    self.dy = 0.;
                }
                Sides::Bottom => {
                    self.y = other.y - self.collision.height() as i32;
                    self.dy = 0.;
                    self.grounded = true;
                }
                Sides::Left => {
                    self.x = other.x + other.width() as i32;
                    self.dx = 0.;
                }
                Sides::Right => {
                    self.x = other.x - self.collision.width() as i32;
                    self.dx = 0.;
                }
                _ => {}
            }
        }
        self.collision.x = self.x;
        self.collision.y = self.y;
    }

    fn collision_side(&self, other: &Rect) -> Sides {
        let amt_right = self.x + self.collision.width() as i32 - other.x;
        let amt_left = other.x + other.width() as i32 - self.x;
        let amt_top = other.y + other.height() as i32 - self.y;
        let amt_bottom = self.y + self.collision.height() as i32 - other.y;

        let mut vals = vec![amt_right.abs(), amt_left.abs(), amt_top.abs(), amt_bottom.abs()];
        vals.sort_unstable();

        let min_val = vals[0];

        if min_val == amt_right.abs() {
            Sides::Right
        } else if min_val == amt_left.abs() {
            Sides::Left
        } else if min_val == amt_top.abs() {
            Sides::Top
        } else if min_val == amt_bottom.abs() {
            Sides::Bottom
        } else {
            Sides::None
        }
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
