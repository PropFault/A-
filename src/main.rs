use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::sync::{Arc, RwLock};
use glam::Vec3;
use rand::Rng;
use sdl2::{Sdl, VideoSubsystem};
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use sdl2::sys::__dev_t;
use crate::ecs::component::Component;
use crate::ecs::ecs::ECS;
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
    points: Vec<Vec3>,
    handle: u64
}

impl Component for Polygon {
    fn get_type_stack_static() -> Vec<u64> where Self: Sized {
        return Vec::from([123995966299]);
    }

    fn get_type_stack(&self) -> Vec<u64> {
        return Polygon::get_type_stack_static();
    }

    fn get_handle(&self) -> u64 {
        return self.handle;
    }

    fn set_handle(&mut self, handle: u64) {
        self.handle = handle;
    }

}


struct PolygonRenderSystem{
    render_kit: Arc<RwLock<SDL2RenderKit>>
}


impl System for PolygonRenderSystem{
    fn handles_component_type_static() -> u64 {
        return *Polygon::get_type_stack_static().first().unwrap();
    }

    fn handles_component_type(&self) -> u64{
        return PolygonRenderSystem::handles_component_type_static();
    }

    fn run(&mut self, components: &mut HashMap<u64, Arc<RwLock<dyn Any>>>) {
        for comp in components.values_mut(){
            let mut compLock = comp.write().unwrap();
            let conc = compLock.downcast_mut::<Polygon>().unwrap();
            for vec in &conc.points{
                println!("{}", vec.to_string());
            }
        }
    }
}


fn main() {
    let mut render_kit = Arc::new(RwLock::new(SDL2RenderKit::new()));
    let mut ecs = ECS::new();
    let mut handleGen = RngHandleGen::new();
    let mut map: HashMap<u64, Arc<RwLock<dyn Any>>> = HashMap::new();
    let mut renderSys = PolygonRenderSystem{render_kit: render_kit.clone() };
    let mut systems = [renderSys];

    let entity = handleGen.gen();
    let polygonOne = handleGen.gen();
    let concPoly = Polygon {points: Vec::from([Vec3::new(1.0, 2.0, 3.9)]), handle:  polygonOne};
    ecs.link(entity, &concPoly).expect("ECS failed");
    map.insert(polygonOne, Arc::new(RwLock::new(concPoly)));

    render_kit.write().unwrap().canvas.clear();
    {
        for mut system in & mut systems{
            let cHandles = ecs.component_types.get(&system.handles_component_type());
            let mut comp: HashMap<u64, Arc<RwLock<dyn Any>>> = HashMap::new();
            for cHandle in cHandles.unwrap(){
                comp.insert(*cHandle, map.get_mut(cHandle).cloned().unwrap());
            }
            system.run(&mut comp);
        }
    }
    render_kit.write().unwrap().canvas.present();

}
