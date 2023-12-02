use sdl2::{Sdl, VideoSubsystem};
use std::sync::RwLock;
use sdl2::render::WindowCanvas;

pub struct SDL2RenderKit{
    pub context : Sdl,
    pub video_subsystem : VideoSubsystem,
    pub canvas: RwLock<WindowCanvas>
}

impl SDL2RenderKit {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let mut conc_canvas = window.into_canvas().build().unwrap();
        let mut canvas = RwLock::new(conc_canvas);

        Self { context: sdl_context, video_subsystem, canvas}
    }
}
