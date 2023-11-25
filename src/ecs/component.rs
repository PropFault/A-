pub trait Component{
    fn get_type_stack(&self) -> Vec<u32>;
    fn get_handle(&self) -> u32;
    fn set_handle(&mut self, handle: u32);
}
