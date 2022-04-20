use sdl2::rect::Rect;

#[derive(Default, Clone)]
pub struct Animation {
    frames: Vec<Rect>,
    frame_index: usize,
    time_elapsed: u32,
    frame_duration: u32,
    play_once: bool,
    pub visible: bool,
}

impl Animation {
    pub fn new(
        rect: Rect,
        frame_length: u32,
        play_once: bool,
        horizontal: usize,
        vertical: usize,
    ) -> Self {
        let mut frames = vec![];

        for y in 0..vertical {
            for x in 0..horizontal {
                let r = Rect::new(
                    rect.x + (rect.width() * x as u32) as i32,
                    rect.y + (rect.height() * y as u32) as i32,
                    rect.width(),
                    rect.height(),
                );
                frames.push(r);
            }
        }

        Self {
            frames,
            frame_index: 0,
            time_elapsed: 0,
            frame_duration: frame_length,
            play_once,
            visible: true,
        }
    }

    pub fn get_current_frame(&self) -> Rect {
        self.frames[self.frame_index]
    }

    pub fn update(&mut self, dt: u32) {
        self.time_elapsed += dt;
        if self.time_elapsed > self.frame_duration {
            self.time_elapsed = 0;
            self.frame_index += 1;
        }

        if self.frame_index >= self.frames.len() {
            self.frame_index = 0;

            self.visible = !self.play_once;
        }
    }
}
