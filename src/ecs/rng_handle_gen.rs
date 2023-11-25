pub struct RngHandleGen{
    rng : Box<dyn rand::Rng>
}

impl RngHandleGen {
    pub fn new() -> Self {
        Self { rng: Box::new(rand::thread_rng()) }
    }
}
