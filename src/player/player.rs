use sdl2::{
    rect::Rect,
    render::{Texture, WindowCanvas},
};

use crate::{
    graphics::{level::Slope, AnimateSprite, Door, Rectangle, Renderable},
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
pub const JUMP_SPPED: f32 = 0.4;

pub struct Player {
    animation: AnimateSprite,
    x: i32,
    y: i32,
    dx: f32,
    dy: f32,
    facing: Direction,
    grounded: bool,
    pub collision: Rect,
    looking_up: bool,
    looking_down: bool,
    pub max_health: i32,
    pub current_health: i32,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        let mut animation = AnimateSprite::new("player".into());
        animation.add_animation("idle_left".into(), Rect::new(0, 0, 16, 16), 150, false, 1, 1);
        animation.add_animation("idle_right".into(), Rect::new(0, 16, 16, 16), 150, false, 1, 1);
        animation.add_animation("move_left".into(), Rect::new(0, 0, 16, 16), 150, false, 3, 1);
        animation.add_animation("move_right".into(), Rect::new(0, 16, 16, 16), 150, false, 3, 1);
        animation.add_animation("idle_left_up".into(), Rect::new(48, 0, 16, 16), 150, false, 1, 1);
        animation.add_animation(
            "idle_right_up".into(),
            Rect::new(48, 16, 16, 16),
            150,
            false,
            1,
            1,
        );
        animation.add_animation("move_left_up".into(), Rect::new(48, 0, 16, 16), 150, false, 3, 1);
        animation.add_animation(
            "move_right_up".into(),
            Rect::new(48, 16, 16, 16),
            150,
            false,
            3,
            1,
        );
        animation.add_animation(
            "look_down_left".into(),
            Rect::new(96, 0, 16, 16),
            150,
            false,
            1,
            1,
        );
        animation.add_animation(
            "look_down_right".into(),
            Rect::new(96, 16, 16, 16),
            150,
            false,
            1,
            1,
        );
        animation.add_animation(
            "look_backwards_left".into(),
            Rect::new(112, 0, 16, 16),
            150,
            false,
            1,
            1,
        );
        animation.add_animation(
            "look_backwards_right".into(),
            Rect::new(112, 16, 16, 16),
            150,
            false,
            1,
            1,
        );

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
            looking_up: false,
            looking_down: false,
            max_health: 3,
            current_health: 2,
        }
    }

    pub fn stop_moving(&mut self) {
        self.dx = 0.;
        self.dy = 0.;
    }

    pub fn jump(&mut self) {
        if self.grounded {
            self.dy = -JUMP_SPPED;
            self.grounded = false;
        }
    }

    pub fn update(&mut self, dt: u32) {
        self.x += (self.dx * dt as f32) as i32;
        // free fall
        if self.dy <= GRAVITY_CAP {
            self.dy += GRAVITY;
        }
        self.y += (self.dy * dt as f32) as i32;

        self.animation.update(dt);
        self.collision.x = self.x;
        self.collision.y = self.y;
    }

    pub fn move_left(&mut self) {
        if self.looking_down && self.grounded {
            return;
        }

        self.dx = WALK_SPEED * -1.;

        if !self.looking_up {
            self.animation.set_animation("move_left".into());
        }
        self.facing = Direction::Left;
    }

    pub fn move_right(&mut self) {
        if self.looking_down && self.grounded {
            return;
        }

        self.dx = WALK_SPEED;
        if !self.looking_up {
            self.animation.set_animation("move_right".into());
        }

        self.facing = Direction::Right;
    }

    pub fn move_vector(&mut self, vector: (i32, i32)) {
        if vector.0 < 0 && self.looking_down && self.grounded {
            return;
        }

        self.dx = WALK_SPEED * vector.0 as f32;
        // self.dy = WALK_SPEED * vector.1 as f32;
    }

    pub fn stop(&mut self) {
        self.dx = 0.0;
        if !self.looking_up && !self.looking_down {
            self.animation.set_animation(if self.facing == Direction::Right {
                "idle_right".into()
            } else {
                "idle_left".into()
            });
        }
        // self.dy = 0.0;
    }

    pub fn handle_tile_collision(&mut self, others: &[Rect]) {
        for other in others {
            let side = self.collision_side(other);
            match side {
                Sides::Top => {
                    self.dy = 0.;
                    self.y = other.y + other.height() as i32;
                    if self.grounded {
                        self.dx = 0.;
                        self.x = ((self.x as f32)
                            - match self.facing {
                                Direction::Right => 1.0,
                                _ => -1.0,
                            }) as i32;
                    }
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

    pub fn handle_slope_collision(&mut self, slopes: &[Slope]) {
        let bounce_rect: Rectangle = self.collision.into();
        let center_x = bounce_rect.center_x();

        for slope in slopes {
            let b = slope.from.top() - slope.get_slope() * slope.from.left().abs();
            let new_y = slope.get_slope() * center_x + b - 4.0;
            if self.grounded {
                self.y = (new_y - bounce_rect.height) as i32;
            }
            self.grounded = true;
        }
    }

    pub fn handle_door_collision(&mut self, doors: &[Door]) -> String {
        let mut level: String = "".into();
        for door in doors {
            if self.grounded && self.looking_down {
                level = door.destination.clone();
            }
        }
        level
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

    /// The player Lookup
    pub fn look_up(&mut self) {
        self.looking_up = true;
        if self.dx == 0. {
            self.animation.set_animation(match self.facing {
                Direction::Right => "idle_right_up".into(),
                _ => "idle_left_up".into(),
            });
        } else {
            self.animation.set_animation(match self.facing {
                Direction::Right => "move_right_up".into(),
                _ => "move_left_up".into(),
            });
        }
    }

    /// The player stops looking up
    pub fn stop_looking_up(&mut self) {
        self.looking_up = false;
    }

    /// The player looks down OR interacts (turns around)
    pub fn look_down(&mut self) {
        self.looking_down = true;
        if self.grounded {
            self.animation.set_animation(match self.facing {
                Direction::Right => "look_backwards_right".into(),
                _ => "look_backwards_left".into(),
            });
        } else {
            self.animation.set_animation(match self.facing {
                Direction::Right => "look_down_right".into(),
                _ => "look_down_left".into(),
            });
        }
    }

    /// The plyaer stops looking down or interacting
    pub fn stop_looking_down(&mut self) {
        self.looking_down = false;
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
