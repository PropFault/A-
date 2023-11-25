use rand::Rng;
use crate::ecs::rng_handle_gen::RngHandleGen;

trait HandleGen{
    fn gen(&mut self) -> u32;
}

impl HandleGen for RngHandleGen{
    fn gen(&mut self) -> u32{
        return self.rng.gen();
    }
}
