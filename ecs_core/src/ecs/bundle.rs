use crate::ecs::{Archetype, ComponentId, ComponentRegistry};

pub trait Bundle {
    fn component_ids(registry: &mut ComponentRegistry) -> Vec<ComponentId>;
    fn insert(self, archetype: &mut Archetype);
}
