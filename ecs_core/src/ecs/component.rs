use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub type ComponentId = u32;

pub trait Component: 'static + Send + Sync {}

pub struct ComponentRegistry {
    next: ComponentId,
    lookup_map: HashMap<TypeId, ComponentId>,
}

impl ComponentRegistry {
    pub fn new() -> Self {
        Self {
            next: 0,
            lookup_map: HashMap::new(),
        }
    }

    pub fn id(&mut self, type_id: TypeId) -> ComponentId {
        self.lookup_map
            .entry(type_id)
            .or_insert({
                let id = self.next;
                self.next += 1;
                id
            })
            .clone()
    }
}

pub struct ComponentList<T: Component> {
    pub components: Vec<T>,
}

impl<T: Component> ComponentList<T> {
    pub fn new() -> Self {
        Self { components: vec![] }
    }
}

pub trait ComponentListOps: 'static + Send + Sync {
    fn len(&self) -> usize;
    fn swap_remove(&mut self, index: usize);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Component> ComponentListOps for ComponentList<T> {
    fn len(&self) -> usize {
        self.components.len()
    }
    fn swap_remove(&mut self, index: usize) {
        self.components.swap_remove(index);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
