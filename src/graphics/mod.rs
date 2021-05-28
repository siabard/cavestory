use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use sdl2::VideoSubsystem;

pub struct Graphics {
    pub canvas: WindowCanvas,
}

impl Graphics {
    pub fn new(video: &VideoSubsystem) -> Graphics {
        let window: Window = video
            .window("isometric", 800, 600)
            .position_centered()
            .resizable()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        Graphics { canvas }
    }
}
