use std::collections::HashMap;

use crate::{
    Component, ComponentId, ComponentList, ComponentListOps, SystemParam,
    ecs::{ComponentRegistry, Entity, EntityRegistry},
};

type EntityIndex = usize;
type ArchetypeIndex = usize;

pub struct World {
    entity_registry: EntityRegistry,
    entities: HashMap<Entity, HashMap<ComponentId, EntityIndex>>,
    entity_lookup: HashMap<(ComponentId, EntityIndex), Entity>,
    component_registry: ComponentRegistry,
    components: Vec<Box<dyn ComponentListOps>>,
    archetype_lookup: HashMap<Entity, ArchetypeIndex>,
    archetypes: HashMap<Vec<ComponentId>, Vec<Entity>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_registry: EntityRegistry::new(),
            entities: HashMap::new(),
            entity_lookup: HashMap::new(),
            component_registry: ComponentRegistry::new(),
            components: Vec::new(),
            archetype_lookup: HashMap::new(),
            archetypes: HashMap::new(),
        }
    }

    /// Retrieves the `ComponentId` from the `ComponentRegistry` for a given Type that implements `Component`
    ///
    /// # Arguments
    /// - `component` Optional instance of the type to get the id
    ///
    /// # Returns
    /// `ComponentId` - The id of the `Component` item
    pub fn component_id<T: Component>(&mut self) -> ComponentId {
        self.component_registry.id::<T>()
    }

    pub fn get_component_mut<T: Component>(&mut self, entity: &Entity) -> Option<&mut T> {
        let id = self.component_registry.id::<T>();
        let index = *self.entities.get(entity)?.get(&id)?;
        self.components[id as usize]
            .as_any_mut()
            .downcast_mut::<ComponentList<T>>()?
            .components
            .get_mut(index)
    }

    pub fn system<P, F>(&mut self, mut f: F)
    where
        for<'w> P: SystemParam<'w>,
        for<'w> F: FnMut(Entity, <P as SystemParam<'w>>::Item),
    {
        let entities: Vec<Entity> = self.entities.keys().cloned().collect();

        for entity in entities {
            if let Some(params) = P::fetch(self, &entity) {
                f(entity, params);
            }
        }
    }

    pub fn entity_system<P, F>(&mut self, entity: &Entity, mut f: F)
    where
        for<'w> P: SystemParam<'w>,
        for<'w> F: FnMut(Entity, <P as SystemParam<'w>>::Item),
    {
        if let Some(params) = P::fetch(self, &entity) {
            f(entity.clone(), params);
        }
    }

    pub fn spawn_entity(&mut self) -> Entity {
        let entity = self.entity_registry.new_entity();
        self.entities.insert(entity.clone(), HashMap::new());
        entity
    }

    pub fn add_component<T: Component>(&mut self, entity: &Entity, component: T) {
        let component_id = self.component_registry.id::<T>();

        // If component already exists for that entity, panic
        if self
            .entities
            .get(&entity)
            .expect("Entity not found in World")
            .contains_key(&component_id)
        {
            panic!("Entity already contains specified component");
        }

        // If component type hasn't been added yet, add it now.
        if component_id as usize >= self.components.len() {
            self.components.push(Box::new(ComponentList::<T>::new()));
        }

        // Get the ids of the entity before new component added. Use this as key for archetype for the entity.
        let mut ids: Vec<ComponentId> = self
            .entities
            .get(&entity)
            .expect("Entity not found in World")
            .keys()
            .cloned()
            .collect();

        // Entity index is the length before inserting entity component.
        let entity_index = self.components[component_id as usize].len();
        self.components[component_id as usize].push_boxed(Box::new(component));

        self.entity_lookup
            .insert((component_id, entity_index), entity.clone());

        // Update entity with its component lookup
        self.entities
            .get_mut(&entity)
            .expect("Entity not found in World")
            .insert(component_id, entity_index);

        // If entity has an archetype lookup, update to the new archetype
        if let Some(archetype_index) = self.archetype_lookup.get(&entity) {
            let entities = self
                .archetypes
                .get_mut(&ids)
                .expect("No entities found for Archetype");
            entities.swap_remove(*archetype_index);

            // If a swapped entity exists at this index, need to update its lookup to reflect the changes
            if *archetype_index < entities.len() {
                let swapped_entity = entities[*archetype_index].clone();
                self.archetype_lookup
                    .insert(swapped_entity, *archetype_index);
            }
        }

        // Finally update the ids to include the new component
        ids.push(component_id);

        // Add the updated entity to its new archetype
        let entities = self.archetypes.entry(ids.clone()).or_insert(Vec::new());

        let archetype_index = entities.len();
        entities.push(entity.clone());

        self.archetype_lookup
            .insert(entity.clone(), archetype_index);
    }

    pub fn despawn_entity(&mut self, entity: Entity) {
        let map = self
            .entities
            .get_mut(&entity)
            .expect("Entity not found in world");

        let ids: Vec<ComponentId> = map.keys().cloned().collect();

        // Remove entity from Archetypes
        if let Some(archetype_index) = self.archetype_lookup.get(&entity) {
            let archetypes = self
                .archetypes
                .get_mut(&ids)
                .expect("Archetype not found for entity");

            archetypes.swap_remove(*archetype_index);

            // Update swapped entities archetype position
            if *archetype_index < archetypes.len() {
                let swapped_entity = archetypes[*archetype_index].clone();
                self.archetype_lookup
                    .insert(swapped_entity, *archetype_index);
            }
        }

        self.archetype_lookup.remove(&entity);

        // Remove Entities Components

        let mut entities_to_update: Vec<(Entity, ComponentId, EntityIndex)> = Vec::new();

        for id in ids {
            let entity_index = map.get(&id).expect("Component not found for Entity");
            self.components[id as usize].swap_remove(*entity_index);
            let old_index = self.components[id as usize].len();
            if old_index == 0 {
                continue;
            }

            // If entity swapped to new index, update it everywhere.
            let updating_entity = self
                .entity_lookup
                .get(&(id, old_index))
                .expect("Entity not found for Component")
                .clone();

            entities_to_update.push((updating_entity.clone(), id, *entity_index));

            self.entity_lookup.remove(&(id, old_index));
            self.entity_lookup
                .insert((id, *entity_index), updating_entity);
        }

        // Update entities that have been victims of swapping
        for (entity, id, index) in entities_to_update {
            self.entities
                .get_mut(&entity)
                .expect("Entity not found in World")
                .insert(id, index);
        }

        self.entities.remove(&entity);

        self.entity_registry.remove_entity(entity);
    }
}

/// Spawns an entity given a `World` and variadic tuple list of `Components`
///
/// # Arguments
/// - `world` - The `World` used to spawn the `Entity`
/// - `(component, component, ..)` - The `Component`'s that make up the `Entity` to be spawned
///
/// # Returns
/// `Entity` - The `Entity` that is created
#[macro_export]
macro_rules! spawn_entity {
    ($world:expr, ($($component:expr),+ $(,)?)) => {{
        let world: &mut ecs_core::World = &mut $world;

        let entity = world.spawn_entity();

        $(
            world.add_component(&entity, $component);
        )+

        entity
    }};
}
