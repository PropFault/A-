pub use std::collections::HashMap;
use crate::ecs::handle_gen::HandleGen;
use crate::ecs::rng_handle_gen::RngHandleGen;

pub struct Atlas<T>{
    pub data: HashMap<u64, Box<T>>,
    gen: RngHandleGen
}


impl<T> Atlas<T>  {
    pub fn new() ->Self {
        Self { data: HashMap::new(), gen: RngHandleGen::new() }
    }
    pub fn insert(&mut self, data: T) -> u64{
        let handle = self.gen.gen();
        self.data.insert(handle, Box::new(data));
        return handle;
    }

    pub fn get(&self, handle: u64) -> Option<&Box<T>> {
        return self.data.get(&handle);
    }

    pub fn get_mut(& mut self, handle: u64) -> Option<&mut Box<T>> {
        return self.data.get_mut(&handle);
    }
}
