use sdl2::rect::Rect;

use crate::player::Direction;

#[derive(Clone, PartialEq, Debug)]
pub enum Sides {
    Left,
    Right,
    Top,
    Bottom,
    None,
}

impl From<Direction> for Sides {
    fn from(direction: Direction) -> Sides {
        match direction {
            Direction::Down => Sides::Bottom,
            Direction::Up => Sides::Top,
            Direction::Left => Sides::Left,
            Direction::Right => Sides::Right,
            Direction::IdleLeft => Sides::Left,
            Direction::IdleRight => Sides::Right,
            _ => Sides::None,
        }
    }
}

pub trait Sided {
    fn get_rect(&self) -> Rect;
    fn get_side(&self, side: Sides) -> i32;
}

impl Sided for Rect {
    fn get_rect(&self) -> Rect {
        *self
    }

    fn get_side(&self, side: Sides) -> i32 {
        match side {
            Sides::Left => self.x,
            Sides::Right => self.x + self.w as i32,
            Sides::Top => self.y,
            Sides::Bottom => self.y + self.h as i32,
            Sides::None => 0,
        }
    }
}

pub fn collides_with(src: &dyn Sided, other: &dyn Sided) -> bool {
    let p1 = src.get_rect();
    let p2 = other.get_rect();

    p1.x < p2.x + p2.w && p1.x + p1.w > p2.x && p1.y < p2.y + p2.h && p1.y + p1.h > p2.y
}
