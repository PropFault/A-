use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub trait System{
    fn handles_component_type_static() -> u64 where Self: Sized;
    fn handles_component_type(&self) -> u64;
    fn run(&mut self, components: &mut HashMap<u64, Arc<RwLock<dyn Any>>>);
}
