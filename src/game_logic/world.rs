use anyhow::Ok;
use ecs_core::World;

use crate::map::Chunk;

pub struct GameWorld {
    pub world: World,
    pub chunk: Chunk,
}

impl GameWorld {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            world: World::new(),
            chunk: Chunk::new([0, 0], 2)?,
        })
    }
}
