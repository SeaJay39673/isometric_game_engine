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

    /// Retrieves the associated id for the type that implements `Component`.
    /// # Arguments
    /// - `component` Optional instance of the type to get the id
    ///
    /// # Returns
    /// `ComponentId` - The id of the Component item
    /// # Panics
    /// If the total number of registered components exceeds the maximum number that can exist
    pub fn id<T: Component>(&mut self) -> ComponentId {
        if self.next == ComponentId::MAX {
            panic!("Exceeding maximum component types in ComponentRegistry");
        }

        let type_id = TypeId::of::<T>();

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
    fn push_boxed(&mut self, item: Box<dyn Any>);
    fn swap_remove(&mut self, index: usize);
    fn at<'a>(&'a mut self, index: usize) -> &'a mut dyn Any;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Component> ComponentListOps for ComponentList<T> {
    fn len(&self) -> usize {
        self.components.len()
    }
    fn at<'a>(&'a mut self, index: usize) -> &'a mut dyn Any {
        &mut self.components[index]
    }
    fn push_boxed(&mut self, item: Box<dyn Any>) {
        let item = *item.downcast::<T>().expect("Component type mismatch");
        self.components.push(item);
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
