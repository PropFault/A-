pub trait System{
    fn handles_component_type() -> u32;
    fn run(&mut self,components: &Vec<u32>);
}
