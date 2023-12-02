use std::any::Any;
use std::collections::HashMap;


use std::sync::{Arc, RwLock};
use std::time::Instant;
use glam::Vec3;
use rand::Rng;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use crate::ecs::component::Component;
use crate::ecs::ecs::ECS;
use crate::ecs::handle_gen::HandleGen;
use crate::ecs::rng_handle_gen::RngHandleGen;
use crate::ecs::system::System;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use ecs::atlas::Atlas;
use sdl2_render_kit::SDL2RenderKit;

mod ecs;
mod sdl2_render_kit;


struct Spatial{
    pos: Vec3,
    global_pos: Vec3,
    handle: u64,
    parent: Option<Arc<RwLock<Spatial>>>,
    child: Vec<Arc<RwLock<Spatial>>>
}

impl Component for Spatial{
    fn get_type_stack_static() -> Vec<u64> where Self: Sized {
        return Vec::from([22391230921]);
    }

    fn get_type_stack(&self) -> Vec<u64> {
        return Spatial::get_type_stack_static();
    }

    fn get_handle(&self) -> u64 {
        return self.handle;
    }

    fn set_handle(&mut self, handle: u64) {
        self.handle = handle;
    }
}

struct Sprite{
    texture_handle: u64,
    rect: Rect,
    handle: u64
}

impl Sprite {
    pub fn new(texture_handle: u64, rect: Rect, handle: u64) -> Self {
        Self { texture_handle, rect, handle }
    }
}

impl Component for Sprite{
    fn get_type_stack_static() -> Vec<u64> where Self: Sized {
        let mut stack = Vec::from([1238581923]);
        stack.append(&mut Spatial::get_type_stack_static());
        return stack;
    }

    fn get_type_stack(&self) -> Vec<u64> {
        return Sprite::get_type_stack_static();
    }

    fn get_handle(&self) -> u64 {
        return self.handle;
    }

    fn set_handle(&mut self, handle: u64) {
        self.handle = handle;
    }
}

struct SpriteRenderSystem<'a>{
    render_kit: Arc<SDL2RenderKit>,
    texture_atlas: Arc<RwLock<Atlas<Texture<'a>>>>
}

impl<'a> System for SpriteRenderSystem<'a>{
    fn handles_component_type_static() -> u64 where Self: Sized {
        return *Sprite::get_type_stack_static().first().unwrap();
    }

    fn handles_component_type(&self) -> u64 {
        return SpriteRenderSystem::handles_component_type_static();
    }

    fn run(&mut self, components: &mut HashMap<u64, Arc<RwLock<dyn Any>>>) {
        for component in components.values_mut() {
            let comp_lock = component.write().unwrap();
            let mut texture_atlas = self.texture_atlas.write().unwrap();
            let mut canvas = self.render_kit.canvas.write().unwrap();
            let conc = comp_lock.downcast_ref::<Sprite>().unwrap();
            canvas.copy(&texture_atlas.get_mut(conc.texture_handle).unwrap(), None, conc.rect).expect("TODO: panic message");
        }
    }
}







fn main() {
    let mut gen = RngHandleGen::new();
    let mut component_atlas: HashMap<u64, Arc<RwLock<dyn Any>>> = HashMap::new();
    let render_kit = Arc::new(SDL2RenderKit::new());
    let texture_creator = Arc::new({
        let l = render_kit.canvas.write().unwrap();
        l.texture_creator()
    });
    let texture_atlas = Arc::new(RwLock::new(Atlas::<Texture>::new()));

    let mut ecs = ECS::new();
    let mut system_atlas: Atlas<Box<dyn System>> = Atlas::new();
    system_atlas.insert(Box::new(SpriteRenderSystem{ render_kit: render_kit.clone(), texture_atlas: texture_atlas.clone() }));

    let doodle_handle = {
        let tex = {

            texture_creator.load_texture("doodle.png").expect("")
        };
        let mut lck = texture_atlas.write().unwrap();
        lck.insert(tex)
    };

    let entity = gen.gen();
    let sprite = Sprite::new(doodle_handle, Rect::new(0, 0, 300, 300), gen.gen());

    ecs.link(entity, &sprite).expect("TODO: panic message");
    component_atlas.insert(sprite.handle, Arc::new(RwLock::new(sprite)));

    let mut run = true;
    let last_clock = Instant::now();
    while run {
        { // Event pump
            let mut pump = render_kit.context.event_pump().unwrap();
            pump.pump_events();
            for event in pump.poll_iter(){
                match event {
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        run = false;
                        break;
                    }
                    _ => ()
                }
            }
        }
        { // System processing and rendering
            render_kit.canvas.write().unwrap().clear();
            for system in & mut system_atlas.data.values_mut(){
                let c_handles = ecs.component_types.get(&system.handles_component_type());
                let mut comp: HashMap<u64, Arc<RwLock<dyn Any>>> = HashMap::new();
                for cHandle in c_handles.unwrap(){
                    comp.insert(*cHandle, component_atlas.get_mut(cHandle).cloned().unwrap());
                }
                system.run(&mut comp);
            }
            render_kit.canvas.write().unwrap().present();
        }
        { // limit fps
            loop{
                let delta = Instant::now() - last_clock;
                if delta.as_millis() > 10 {
                    break;
                }
            }
        }
    }

}
