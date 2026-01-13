type EntityId = u32;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Entity {
    id: EntityId,
    generation: EntityId,
}

pub struct EntityRegistry {
    free: Vec<Entity>,
    next: EntityId,
}

impl EntityRegistry {
    pub fn new() -> Self {
        Self {
            free: vec![],
            next: 0,
        }
    }

    pub fn new_entity(&mut self) -> Entity {
        if let Some(mut free) = self.free.pop() {
            if free.generation != EntityId::MAX {
                free.generation += 1;
                return free;
            }
        }

        if self.next == EntityId::MAX {
            panic!("Maximum number of entities exceeded");
        }

        let id = self.next;
        self.next += 1;
        Entity { id, generation: 0 }
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        self.free.push(entity);
    }
}
