use cavestory::{game::Game, graphics::Graphics, input::Input};
use sdl2::{event::Event, image::InitFlag, keyboard::Scancode, EventPump, Sdl, VideoSubsystem};

fn main() {
    let context: Sdl = sdl2::init().unwrap();
    let video: VideoSubsystem = context.video().unwrap();
    let mut event_pump: EventPump = context.event_pump().unwrap();
    let mut input: Input = Input::default();
    let mut timer = context.timer().unwrap();

    let window: sdl2::video::Window = video
        .window("isometric", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut texture_creator = canvas.texture_creator();

    // add PNG / JPEG support
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG);

    let mut last_update_time: u32 = timer.ticks();
    let mut current_time: u32;
    let mut dt: u32;

    let mut game = Game::new();
    game.init_sprite(&mut texture_creator);

    'running: loop {
        input.begin_new_frame();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { repeat, .. } if !repeat => {
                    input.key_down_event(&event);
                }
                Event::KeyUp { .. } => {
                    input.key_up_event(&event);
                }
                _ => {}
            }
        }
        if input.was_key_pressed(Scancode::Escape) {
            break 'running;
        }

        current_time = timer.ticks();
        dt = current_time - last_update_time;

        canvas.clear();

        game.render(&mut canvas);

        canvas.present();

        last_update_time = current_time;
    }
}
