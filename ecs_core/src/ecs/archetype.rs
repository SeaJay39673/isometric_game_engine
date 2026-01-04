use std::any::TypeId;

use crate::{
    Component, ComponentList,
    ecs::{ComponentId, ComponentListOps, Entity},
};

pub struct Archetype {
    component_types: Vec<ComponentId>,
    entities: Vec<Entity>,
    components: Vec<Box<dyn ComponentListOps>>,
}

impl Archetype {
    pub fn new(component_ids: &[ComponentId]) -> Self {
        let mut component_types: Vec<ComponentId> = vec![];
        component_types.extend_from_slice(component_ids);
        Self {
            component_types,
            entities: vec![],
            components: vec![],
        }
    }

    pub fn has_components(&self, component_ids: &[ComponentId]) -> bool {
        for id in component_ids {
            if !self.component_types.contains(&id) {
                return false;
            }
        }

        true
    }

    pub fn push<T: Component>(&mut self, component: T) {
        let mut column_index = None;

        for (i, col) in self.components.iter().enumerate() {
            if col.as_any().is::<ComponentList<T>>() {
                column_index = Some(i);
                break;
            }
        }

        let idx = if let Some(index) = column_index {
            index
        } else {
            let list: ComponentList<T> = ComponentList::new();
            self.components.push(Box::new(list));
            self.components.len() - 1
        };

        if let Some(comp_list) = self.components[idx]
            .as_any_mut()
            .downcast_mut::<ComponentList<T>>()
        {
            comp_list.components.push(component);
        }
    }

    pub fn push_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn entity_index(&self, entity: &Entity) -> Option<usize> {
        self.entities
            .iter()
            .enumerate()
            .find(|(i, ent)| *ent == entity)
            .map(|(i, _)| i)
    }

    pub fn remove_entity(&mut self, index: usize) {
        self.components.swap_remove(index);
        self.entities.swap_remove(index);
    }
}
