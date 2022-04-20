use sdl2::{
    rect::Rect,
    render::{Texture, WindowCanvas},
};

use crate::graphics::{AnimateSprite, Renderable};

use super::{Direction, Player};

pub trait Enemy {
    fn update(&mut self, dt: u32, player: &Player);
    fn set_animation(&mut self, animation: String);
    fn add_animation(
        &mut self,
        name: String,
        rect: Rect,
        duration: u32,
        play_once: bool,
        horizontal: usize,
        vertical: usize,
    );
    fn get_collision(&self) -> Rect;
}

pub trait EnemyRenderable: Enemy + Renderable {
    fn to_renderable(&self) -> Option<&dyn Renderable>;
    fn to_enemy(&self) -> Option<&dyn Enemy>;
    fn to_enemy_mut(&mut self) -> Option<&mut dyn Enemy>;
}

#[derive(Clone)]
pub struct Bat {
    id: uuid::Uuid,
    animation: AnimateSprite,
    x: i32,
    y: i32,
    starting_x: i32,
    starting_y: i32,
    should_move_up: bool,
    dx: f32,
    dy: f32,
    facing: Direction,
    pub collision: Rect,
}

impl Bat {
    pub fn new(x: i32, y: i32) -> Self {
        let animation: AnimateSprite = AnimateSprite::new("enemy".into());
        // animation.add_animation("fly_left".into(), Rect::new(2, 32, 16, 16), 150, false, 3, 1);
        // animation.add_animation("fly_right".into(), Rect::new(2, 48, 16, 16), 150, false, 3, 1);

        Self {
            id: uuid::Uuid::new_v4(),
            animation,
            x,
            y,
            starting_x: x,
            starting_y: y,
            should_move_up: false,
            dx: 0.,
            dy: 0.,
            facing: Direction::Idle,
            collision: Rect::new(0, 0, 16, 16),
        }
    }
}

impl Enemy for Bat {
    fn update(&mut self, dt: u32, player: &Player) {
        // move bat
        self.dy = if self.should_move_up { -0.08 } else { 0.08 };
        self.x = ((self.x as f32) + self.dx * dt as f32) as i32;
        self.y = ((self.y as f32) + self.dy * dt as f32) as i32;

        if self.y > (self.starting_y + 20) || self.y < (self.starting_y - 20) {
            self.should_move_up = !self.should_move_up;
        }

        self.facing = if player.x > self.x { Direction::Right } else { Direction::Left };
        self.set_animation(if self.facing == Direction::Right {
            "fly_right".into()
        } else {
            "fly_left".into()
        });
        self.animation.update(dt);
    }

    fn set_animation(&mut self, animation: String) {
        self.animation.set_animation(animation);
    }

    fn add_animation(
        &mut self,
        name: String,
        rect: Rect,
        duration: u32,
        play_once: bool,
        horizontal: usize,
        vertical: usize,
    ) {
        self.animation.add_animation(name, rect, duration, play_once, horizontal, vertical);
    }

    fn get_collision(&self) -> Rect {
        Rect::new(self.x, self.y, self.collision.width(), self.collision.height())
    }
}

impl Enemy for Box<Bat> {
    fn update(&mut self, dt: u32, player: &Player) {
        // move bat
        self.dy = if self.should_move_up { -0.08 } else { 0.08 };
        self.x = ((self.x as f32) + self.dx * dt as f32) as i32;
        self.y = ((self.y as f32) + self.dy * dt as f32) as i32;
        if self.y > (self.starting_y + 20) || self.y < (self.starting_y - 20) {
            self.should_move_up = !self.should_move_up;
        }

        self.facing = if player.x > self.x { Direction::Right } else { Direction::Left };
        self.set_animation(if self.facing == Direction::Right {
            "fly_right".into()
        } else {
            "fly_left".into()
        });
        self.animation.update(dt);
    }

    fn set_animation(&mut self, animation: String) {
        self.animation.set_animation(animation);
    }

    fn add_animation(
        &mut self,
        name: String,
        rect: Rect,
        duration: u32,
        play_once: bool,
        horizontal: usize,
        vertical: usize,
    ) {
        self.animation.add_animation(name, rect, duration, play_once, horizontal, vertical);
    }

    fn get_collision(&self) -> Rect {
        Rect::new(self.x, self.y, self.collision.width(), self.collision.height())
    }
}

impl Renderable for Bat {
    fn get_name(&self) -> String {
        "enemy".into()
    }

    fn render(&self, canvas: &mut WindowCanvas, texture: &Texture) {
        self.animation.render(self.x, self.y, canvas, texture);
    }
}

impl Renderable for Box<Bat> {
    fn get_name(&self) -> String {
        "enemy".into()
    }

    fn render(&self, canvas: &mut WindowCanvas, texture: &Texture) {
        self.animation.render(self.x, self.y, canvas, texture);
    }
}

impl EnemyRenderable for Bat {
    fn to_renderable(&self) -> Option<&dyn Renderable> {
        Some(self)
    }

    fn to_enemy(&self) -> Option<&dyn Enemy> {
        Some(self)
    }

    fn to_enemy_mut(&mut self) -> Option<&mut dyn Enemy> {
        Some(self)
    }
}
