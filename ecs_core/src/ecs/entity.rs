#[derive(Clone, PartialEq, Eq)]
pub struct Entity {
    id: u32,
    generation: u32,
}

pub struct EntityRegistry {
    free: Vec<Entity>,
    next: u32,
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
            free.generation += 1;
            return free;
        }

        let id = self.next;
        self.next += 1;
        Entity { id, generation: 0 }
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        self.free.push(entity);
    }
}
