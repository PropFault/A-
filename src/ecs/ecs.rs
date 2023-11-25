use std::collections::{HashMap, HashSet};
use crate::component::Component;

pub struct ECS{
    component_to_entity:HashMap<u32,u32>,
    entity_to_component:HashMap<u32, HashSet<u32>>,
    component_types: HashMap<u32, HashSet<u32>>
}

impl ECS{
    fn link(&mut self, entity: u32, component: &dyn Component) -> Result<&mut ECS, ECSErrors>{
        let c_type = component.get_type_stack();
        let c_handle = component.get_handle();

        if self.component_to_entity.contains_key(&c_handle) {
            return Err(ECSErrors::ComponentAlreadyLinked);
        }

        self.component_to_entity.insert(c_handle, entity);

        if !self.entity_to_component.contains_key(&entity){
            self.entity_to_component.insert(entity, HashSet::new());
        }
        self.entity_to_component.get_mut(&entity).unwrap().insert(c_handle);

        for t in c_type {
            if !self.component_types.contains_key(&t){
                self.component_types.insert(t, HashSet::new());
            }
            self.component_types.get_mut(&t).unwrap().insert(c_handle);
        }
        return Ok(self);
    }

    fn unlink_component(&mut self, component: &dyn Component){
        let c_type = component.get_type_stack();
        let c_handle = component.get_handle();
        self.component_to_entity.remove(&c_handle);
        for t in c_type {
            self.component_types.get_mut(&t).unwrap().remove(&c_handle);
        }
    }
    pub fn new() -> Self {
        Self { component_to_entity: HashMap::new(),
            entity_to_component: HashMap::new(),
            component_types: HashMap::new() }
    }
}

enum ECSErrors{
    ComponentAlreadyLinked
}
