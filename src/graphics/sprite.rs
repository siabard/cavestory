use sdl2::rect::Rect;

pub struct Sprite {
    pub name: String,
    pub source_rect: Rect,
    x: f64,
    y: f64,
}

impl Sprite {
    pub fn new(
        name: String,
        src_x: i32,
        src_y: i32,
        width: u32,
        height: u32,
        pos_x: f64,
        pos_y: f64,
    ) -> Sprite {
        let source_rect = Rect::new(src_x, src_y, width, height);
        Sprite {
            name,
            source_rect,
            x: pos_x,
            y: pos_y,
        }
    }
}
