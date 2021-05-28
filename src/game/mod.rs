use crate::graphics::Graphics;
use crate::input::Input;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::{EventPump, Sdl, TimerSubsystem, VideoSubsystem};

const FPS: u32 = 50;
const MAX_FRAME_TIME: u32 = 5 * 1000 / FPS;

pub struct Game {
    event_pump: EventPump,
    graphics: Graphics,
    timer: TimerSubsystem,
    input: Input,
}

impl Game {
    pub fn new() -> Game {
        let context: Sdl = sdl2::init().unwrap();
        let video: VideoSubsystem = context.video().unwrap();
        let event_pump: EventPump = context.event_pump().unwrap();
        let graphics: Graphics = Graphics::new(&video);
        let input: Input = Input::default();
        let timer = context.timer().unwrap();

        Game {
            event_pump,
            graphics,
            input,
            timer,
        }
    }

    pub fn game_loop(&mut self) {
        let mut last_update_time: u32 = self.timer.ticks();
        let mut current_time: u32;
        let mut dt: u32;
        'running: loop {
            self.input.begin_new_frame();

            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::KeyDown { repeat, .. } if !repeat => {
                        self.input.key_down_event(&event);
                    }
                    Event::KeyUp { .. } => {
                        self.input.key_up_event(&event);
                    }
                    _ => {}
                }
            }
            if self.input.was_key_pressed(Scancode::Escape) == true {
                break 'running;
            }

            current_time = self.timer.ticks();
            dt = current_time - last_update_time;

            self.update(dt.min(MAX_FRAME_TIME));

            self.draw();

            last_update_time = current_time;
        }
    }

    fn update(&mut self, elapsed_time: u32) {
        println!("dt : {}", elapsed_time);
    }

    fn draw(&mut self) {
        self.graphics.canvas.clear();
        self.graphics.canvas.present();
    }
}
