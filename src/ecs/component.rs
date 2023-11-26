use std::any::Any;

pub trait Component{
    fn get_type_stack_static() -> Vec<u64> where Self: Sized;
    fn get_type_stack(&self) -> Vec<u64>;
    fn get_handle(&self) -> u64;
    fn set_handle(&mut self, handle: u64);
}
