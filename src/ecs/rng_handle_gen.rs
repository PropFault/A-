use rand::Rng;
use rand::rngs::ThreadRng;
use ecs::handle_gen::HandleGen;
use crate::ecs;

pub struct RngHandleGen{
    rng : ThreadRng
}

impl RngHandleGen {
    pub fn new() -> Self {
        Self { rng: rand::thread_rng() }
    }
}

impl HandleGen for RngHandleGen{
    fn gen(&mut self) -> u32{
        return self.rng.gen();
    }
}