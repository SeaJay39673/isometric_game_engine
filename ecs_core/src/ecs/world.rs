use crate::{
    Bundle,
    ecs::{Archetype, ComponentRegistry, Entity, EntityRegistry},
};

pub struct World {
    entity_registry: EntityRegistry,
    component_registry: ComponentRegistry,
    archetypes: Vec<Archetype>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_registry: EntityRegistry::new(),
            component_registry: ComponentRegistry::new(),
            archetypes: vec![],
        }
    }

    pub fn spawn_entity<B: Bundle>(&mut self, bundle: B) -> Entity {
        let entity = self.entity_registry.new_entity();

        let component_ids = B::component_ids(&mut self.component_registry);

        let archetype = match self
            .archetypes
            .iter_mut()
            .find(|a| a.has_components(&component_ids))
        {
            Some(archetype) => archetype,
            None => {
                let archetype = Archetype::new(&component_ids);
                self.archetypes.push(archetype);
                self.archetypes.last_mut().unwrap()
            }
        };

        archetype.push_entity(entity.clone());

        bundle.insert(archetype);

        entity
    }

    pub fn despawn(&mut self, entity: Entity) {
        for archetype in &mut self.archetypes {
            if let Some(index) = archetype.entity_index(&entity) {
                archetype.remove_entity(index);
                self.entity_registry.remove_entity(entity);
                break;
            }
        }
    }
}
