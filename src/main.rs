use glam::Vec3;
use rand::Rng;
use sdl2::{Sdl, VideoSubsystem};
use sdl2::render::WindowCanvas;
use crate::ecs::component::Component;
use crate::ecs::handle_gen::HandleGen;
use crate::ecs::rng_handle_gen::RngHandleGen;
use crate::ecs::system::System;

mod ecs;

struct SDL2RenderKit{
    context : Sdl,
    video_subsystem : VideoSubsystem,
    canvas: WindowCanvas
}

impl SDL2RenderKit {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        Self { context: sdl_context, video_subsystem, canvas }
    }
}

struct Polygon {
    points: Vec<Vec3>
}

impl Component for Polygon {
    fn get_type_stack(&self) -> Vec<u32> {
        let stack = self.get_type_stack();
        let gen = RngHandleGen::new();
    }

    fn get_handle(&self) -> u32 {
        todo!()
    }

    fn set_handle(&mut self, handle: u32) {
        todo!()
    }
}


struct PolygonRenderSystem{
    render_kit:& SDL2RenderKit
}

impl System for PolygonRenderSystem{
    fn handles_component_type(){
        return
    }

    fn run(&mut self, components: &Vec<u32>) {
        todo!()
    }
}

fn main() {
    let renderKit = SDL2RenderKit::new();

}
